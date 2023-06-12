use network_tables::v4::client_config::default_should_reconnect;
use network_tables::v4::subscription::SubscriptionOptions;
use network_tables::v4::{Client, Config, PublishedTopic, Subscription, Type};
use std::collections::HashMap;
use std::hash::Hash;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::task::JoinHandle as TokioJoinHandle;

use crate::mushroom_types::{MushroomTable, MushroomEntryValue};
use crate::THREAD_POOL;

#[derive(Debug, serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct NetworkTableHandlerId {
    ip: Ipv4Addr,
    port: u16,
    identity: String,
}
impl NetworkTableHandlerId {
    pub fn new(ip: Ipv4Addr, port: u16, identity: String) -> Self {
        Self { ip, port, identity }
    }
}

#[derive(Debug)]
pub struct NetworkTableHandler {
    id: NetworkTableHandlerId,
    subscriptions: Sender<Vec<SubscriptionData>>,
    input: Sender<MushroomTable>,
    output: Receiver<MushroomTable>,
    thread: TokioJoinHandle<()>,
}
impl NetworkTableHandler {
    fn new(
        id: NetworkTableHandlerId,
        subscriptions: Sender<Vec<SubscriptionData>>,
        input: Sender<MushroomTable>,
        output: Receiver<MushroomTable>,
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

    pub fn post(&mut self, table: MushroomTable) {
        self.input
            .try_send(table)
            .unwrap_or_else(|err| {
                tracing::error!(
                    "Failed to send to network table handler {}:{}",
                    self.id.ip,
                    self.id.port
                );
                tracing::error!("Error: {}", err);
            });
    }

    pub fn subscribe(&mut self, sub_data: Vec<SubscriptionData>) {
        self.subscriptions
            .try_send(sub_data)
            .unwrap_or_else(|err| {
                tracing::error!(
                    "Failed to send to network table handler {}:{}",
                    self.id.ip,
                    self.id.port
                );
                tracing::error!("Error: {}", err);
            });
    }

    pub fn poll(&mut self) -> Option<MushroomTable> {
        self.output.try_recv().ok()
    }

    pub fn get_id(&self) -> &NetworkTableHandlerId {
        &self.id
    }
}

#[derive(Debug)]
struct SubscriptionData {
    name: String,
    options: Option<SubscriptionOptions>,
}
impl Hash for SubscriptionData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

pub fn nt4(
    address: Ipv4Addr,
    port: u16,
    identity: String,
) -> Result<NetworkTableHandler, Box<dyn std::error::Error>> {
    let (snd_pub, rec_pub) = channel::<MushroomTable>(255);
    let (snd_sub, rec_sub) = channel::<MushroomTable>(255);
    let (subscription_sender, subscription_receiver) = channel::<Vec<SubscriptionData>>(255);
    let id = NetworkTableHandlerId {
        ip: address,
        port,
        identity: identity.clone(),
    };
    let thread = inner_nt4(address, port, identity, subscription_receiver, rec_pub, snd_sub)?;
    let handler = NetworkTableHandler::new(id, subscription_sender, snd_pub, rec_sub, thread);

    Ok(handler)
}

fn inner_nt4(
    address: Ipv4Addr,
    port: u16,
    identity: String,
    mut subscriptions: Receiver<Vec<SubscriptionData>>,
    mut input: Receiver<MushroomTable>,
    output: Sender<MushroomTable>,
    ) -> Result<TokioJoinHandle<()>, Box<dyn std::error::Error>> {
    let thread = THREAD_POOL.with(|thread_pool| {
        thread_pool.spawn(async move {
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
            ).await.unwrap();

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
                                    tracing::error!(
                                        "Failed to subscribe to {}:{}",
                                        address,
                                        port
                                    );
                                    tracing::error!("Error: {}", err);
                                    panic!();
                                });
                            subs.insert(name.clone(), sub);
                            tracing::info!("Subscribed to {}:{}:{}", address, port, name);
                    }
                }

                let new_pub_data = input.try_recv();
                if let Ok(table) = new_pub_data {
                    for entry in table {
                        let path = entry.get_path_string();
                        if !pubs.contains_key(&path) {
                            let topic = client.publish_topic(path.as_str(), Type::from(entry.get_value()), None).await.unwrap();
                            pubs.insert(path.clone(), topic);
                        }
                        let topic = pubs.get(&path).unwrap();
                        client.publish_value(topic, &rmpv::Value::from(entry.get_value())).await.ok();
                        tracing::info!("Published to {}:{}:{}", address, port, path);
                    }
                }

                let mut table_data: MushroomTable = Vec::new();
                for sub in subs.values_mut() {
                    while let Some(msg) = sub.next().await {
                        let entry = MushroomEntryValue::new(
                            msg.data.into(),
                            MushroomEntryValue::make_path(msg.topic_name.as_str()),
                            Some(msg.timestamp as u64)
                        );
                        table_data.push(entry);
                    }
                }
                if !table_data.is_empty() {
                    output.try_send(table_data).unwrap_or_else(|err| {
                        tracing::error!(
                            "Failed to send to network table handler {}:{}",
                            address,
                            port
                        );
                        tracing::error!("Error: {}", err);
                    });
                }

                let elapsed = start_time.elapsed();
                tokio::time::sleep(
                    Duration::from_secs_f64((Duration::from_millis(15) - elapsed).as_secs_f64().clamp(0.0, 0.015))).await;

            }

    })});
    Ok(thread)
}
