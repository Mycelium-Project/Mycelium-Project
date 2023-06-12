use network_tables::v4::client_config::default_should_reconnect;
use network_tables::v4::subscription::SubscriptionOptions;
use network_tables::v4::{Client, Config, PublishedTopic, Subscription, Type};
use single_value_channel::{Updater as SingleUpdater, channel_starting_with as single_channel, Receiver as SingleReceiver};
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::task::JoinHandle as TokioJoinHandle;


use crate::mushroom_types::{MushroomEntry, MushroomTable};
use crate::THREAD_POOL;

#[derive(Debug, serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct NetworkTableHandlerId {
    ip: [u8; 4],
    port: u16,
    identity: String,
}
impl NetworkTableHandlerId {
    pub fn new(ip: Ipv4Addr, port: u16, identity: String) -> Self {
        Self { ip: ip.octets(), port, identity }
    }
}
impl Display for NetworkTableHandlerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", Ipv4Addr::new(self.ip[0], self.ip[1], self.ip[2], self.ip[3]), self.port, self.identity)
    }
}

#[derive(Debug)]
pub struct NetworkTableHandler {
    id: NetworkTableHandlerId,
    subscriptions: Sender<Vec<SubscriptionPackage>>,
    input: Sender<MushroomTable>,
    output: SingleReceiver<MushroomTable>,
    thread: TokioJoinHandle<()>
}
impl NetworkTableHandler {
    fn new(
        id: NetworkTableHandlerId,
        subscriptions: Sender<Vec<SubscriptionPackage>>,
        input: Sender<MushroomTable>,
        output: SingleReceiver<MushroomTable>,
        thread: TokioJoinHandle<()>,
    ) -> Self {
        Self {
            id,
            subscriptions,
            input,
            output,
            thread,
        }
    }

    pub fn stop(&self) {
        self.thread.abort();
    }

    pub fn publish(&mut self, table: MushroomTable) {
        self.input.try_send(table).unwrap_or_else(|err| {
            tracing::error!(
                "Failed to send to network table handler {}",
                self.id
            );
            tracing::error!("Error: {}", err);
        });
    }

    pub fn subscribe(&mut self, sub_data: Vec<SubscriptionPackage>) {
        self.subscriptions.try_send(sub_data).unwrap_or_else(|err| {
            tracing::error!(
                "Failed to send to network table handler {}",
                self.id
            );
            tracing::error!("Error: {}", err);
        });
    }

    pub fn poll(&mut self) -> MushroomTable {
        self.output.latest().clone()
    }

    // pub fn get_id(&self) -> &NetworkTableHandlerId {
    //     &self.id
    // }
}

#[derive(Debug)]
pub struct SubscriptionPackage {
    name: String,
    options: Option<SubscriptionOptions>,
}
impl Hash for SubscriptionPackage {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
impl SubscriptionPackage {
    pub fn new(name: String, options: SubscriptionOptions) -> Self {
        Self {
            name,
            options: Some(options),
        }
    }
}

pub fn nt4(
    address: Ipv4Addr,
    port: u16,
    identity: String,
) -> Result<NetworkTableHandler, Box<dyn std::error::Error>> {
    let (snd_pub, rec_pub) = channel::<MushroomTable>(255);
    let (rec_sub, snd_sub) = single_channel(MushroomTable::new(0));
    let (subscription_sender, subscription_receiver) = channel::<Vec<SubscriptionPackage>>(255);
    let id = NetworkTableHandlerId {
        ip: address.octets(),
        port,
        identity: identity.clone(),
    };
    let thread = inner_nt4(
        address,
        port,
        identity,
        subscription_receiver,
        rec_pub,
        snd_sub,
    )?;
    let handler = NetworkTableHandler::new(id, subscription_sender, snd_pub, rec_sub, thread);

    Ok(handler)
}

fn inner_nt4(
    address: Ipv4Addr,
    port: u16,
    identity: String,
    mut subscriptions: Receiver<Vec<SubscriptionPackage>>,
    mut input: Receiver<MushroomTable>,
    output: SingleUpdater<MushroomTable>,
) -> Result<TokioJoinHandle<()>, Box<dyn std::error::Error>> {
    let thread = THREAD_POOL.with(|thread_pool| {
        thread_pool.borrow().as_ref().unwrap().spawn(async move {
            let mut subs: HashMap<String, Subscription> = HashMap::new();
            let mut pubs: HashMap<String, PublishedTopic> = HashMap::new();

            let client: Client = Client::try_new_w_config(
                SocketAddrV4::new(address, port),
                Config {
                    connect_timeout: 5000,
                    disconnect_retry_interval: 10000,
                    should_reconnect: Box::new(default_should_reconnect),
                    on_announce: Box::new(|_| {
                        Box::pin(async {
                            tracing::info!("Announced");
                        })
                    }),
                    on_un_announce: Box::new(|_| {
                        Box::pin(async {
                            tracing::info!("Un-announced");
                        })
                    }),
                    on_disconnect: Box::new(|| {
                        Box::pin(async {
                            tracing::info!("Disconnected");
                        })
                    }),
                    on_reconnect: Box::new(|| {
                        Box::pin(async {
                            tracing::info!("Reconnected");
                        })
                    }),
                },
                Option::from("Enoki"),
            )
            .await
            .unwrap();

            //use client timestamp
            let mut table = MushroomTable::new(0);

            loop {
                let start_time = std::time::Instant::now();

                let new_sub_data = subscriptions.try_recv();
                if let Ok(new_sub_data) = new_sub_data {
                    for sub_data in new_sub_data {
                        let name = sub_data.name.clone();
                        let options = sub_data.options.clone();
                        if subs.contains_key(&name) {
                            client.unsubscribe(subs.remove(&name).unwrap()).await.ok();
                        }
                        let sub = client
                            .subscribe_w_options(&[name.clone()], options)
                            .await
                            .unwrap_or_else(|err| {
                                tracing::error!("Failed to subscribe to {}:{}", address, port);
                                tracing::error!("Error: {}", err);
                                panic!();
                            });
                        subs.insert(name.clone(), sub);
                        tracing::info!("Subscribed to {}:{}:{}", address, port, name);
                    }
                }

                let new_pub_data = input.try_recv();
                if let Ok(table) = new_pub_data {
                    for entry in table.get_entries() {
                        let path = String::from(entry.get_path());
                        if !pubs.contains_key(&path) {
                            let topic = client
                                .publish_topic(path.as_str(), Type::from(entry.get_value()), None)
                                .await
                                .unwrap();
                            pubs.insert(path.clone(), topic);
                        }
                        let topic = pubs.get(&path).unwrap();
                        client
                            .publish_value(topic, &rmpv::Value::from(entry.get_value()))
                            .await
                            .ok();
                        tracing::info!("Published to {}:{}:{}", address, port, path);
                    }
                }

                //use client timestamp
                let mut new_table_data: MushroomTable = MushroomTable::new(0);
                for sub in subs.values_mut() {
                    while let Some(msg) = sub.next().await {
                        let entry = MushroomEntry::new(
                            msg.data.into(),
                            msg.topic_name.into(),
                            Some(msg.timestamp as f64),
                        );
                        new_table_data.add_entry(entry)
                    }
                }
                table.update_all(&new_table_data);
                if !new_table_data.is_empty() {
                    output.update(table.clone()).unwrap_or_else(|err| {
                        tracing::error!(
                            "Failed to send to network table handler {}:{}",
                            address,
                            port
                        );
                        tracing::error!("Error: {}", err);
                    });
                }

                let elapsed = start_time.elapsed();
                tokio::time::sleep(Duration::from_secs_f64(
                    (Duration::from_millis(15) - elapsed)
                        .as_secs_f64()
                        .clamp(0.0, 0.015),
                ))
                .await;
            }
        })
    });
    Ok(thread)
}
