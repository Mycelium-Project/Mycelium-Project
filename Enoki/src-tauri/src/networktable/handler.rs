use network_tables::v4::client::clear_identity;
use network_tables::v4::client_config::default_should_reconnect;
use network_tables::v4::subscription::SubscriptionOptions;
use network_tables::v4::{Client, Config, PublishedTopic, Subscription, Type};
use single_value_channel::{
    channel_starting_with as single_channel, Receiver as SingleReceiver, Updater as SingleUpdater,
};
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4};
use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::task::JoinHandle as TokioJoinHandle;

use crate::datalog::DATALOG;
use crate::enoki_types::{now, EnokiField, EnokiObject, TimestampedEnokiValue, EnokiKey, EnokiTimeStamp};
use crate::error::{EnokiError, log_result_consume};
use crate::NETWORK_CLIENT_MAP;

pub fn get_connect_client_names() -> Vec<String> {
    let mut names = Vec::new();
    for (name, _) in NETWORK_CLIENT_MAP.lock().iter() {
        names.push(name.repr());
    }
    names
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct NetworkTableClientId {
    pub(super) ip: [u8; 4],
    pub(super) port: u16,
    pub(super) identity: String,
}
impl NetworkTableClientId {
    pub fn new(ip: Ipv4Addr, port: u16, identity: String) -> Self {
        Self {
            ip: ip.octets(),
            port,
            identity,
        }
    }

    pub fn repr(&self) -> String {
        format!("{}", self)
    }
}
impl Display for NetworkTableClientId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            Ipv4Addr::new(self.ip[0], self.ip[1], self.ip[2], self.ip[3]),
            self.port,
            self.identity
        )
    }
}

#[derive(Debug)]
pub struct NetworkTableClient {
    id: NetworkTableClientId,
    subscriptions: Sender<SubscriptionPackage>,
    input: Sender<EnokiObject>,
    output: SingleReceiver<HashMap<String, EnokiObject>>,
    thread: TokioJoinHandle<()>,
}
impl NetworkTableClient {
    fn new(
        id: NetworkTableClientId,
        subscriptions: Sender<SubscriptionPackage>,
        input: Sender<EnokiObject>,
        output: SingleReceiver<HashMap<String, EnokiObject>>,
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
        clear_identity(self.id.identity.clone());
        self.thread.abort();
    }

    pub fn publish_obj(&mut self, obj: EnokiObject) {
        tracing::info!("Publishing table to network table client {}", self.id);
        self.input.try_send(obj).unwrap_or_else(|err| {
            tracing::error!(
                "Failed to publish to network table client {} because {}",
                self.id,
                err
            );
        });
    }

    pub fn publish_field(&mut self, field: EnokiField) {
        tracing::info!("Publishing field to network table client {}", self.id);
        self.input.try_send(EnokiObject::from_field(field)).unwrap_or_else(|err| {
            tracing::error!(
                "Failed to publish to network table client {} because {}",
                self.id,
                err
            );
        });
    }

    pub fn subscribe(&mut self, sub_data: SubscriptionPackage) {
        self.subscriptions.try_send(sub_data).unwrap_or_else(|err| {
            tracing::error!(
                "Failed to subscrive to network table client {} because {}",
                self.id,
                err
            );
        });
    }

    pub fn unsubscribe(&mut self, topic: String) {
        self.subscriptions
            .try_send(SubscriptionPackage::unsub(topic))
            .unwrap_or_else(|err| {
                tracing::error!(
                    "Failed to unsubscribe to network table client {} because {}",
                    self.id,
                    err
                );
            });
    }

    pub fn poll(&mut self, topic: String) -> Result<EnokiObject, EnokiError> {
        if let Some(sub_obj) = self.output.latest().get(&topic) {
            Ok(sub_obj.clone())
        } else {
            Err(EnokiError::NTTopicNotFound(topic))
        }
    }

    pub fn poll_all(&mut self) -> Vec<EnokiObject> {
        self.output.latest().clone().values().cloned().collect()
    }
}

#[derive(Debug)]
pub struct SubscriptionPackage {
    topic: String,
    unsubscribe: bool,
    options: Option<SubscriptionOptions>,
}
impl Hash for SubscriptionPackage {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.topic.hash(state);
    }
}
impl SubscriptionPackage {
    pub fn new(name: String, options: SubscriptionOptions) -> Self {
        Self {
            topic: name,
            unsubscribe: false,
            options: Some(options),
        }
    }

    pub fn unsub(name: String) -> Self {
        Self {
            topic: name,
            unsubscribe: true,
            options: None,
        }
    }
}

pub fn datalog_type(nt_type: &Type) -> String {
    match nt_type.to_owned() {
        Type::Boolean => "boolean".to_string(),
        Type::Double => "double".to_string(),
        Type::String => "string".to_string(),
        Type::Float => "float".to_string(),
        Type::Int => "int64".to_string(),
        Type::BooleanArray => "boolean[]".to_string(),
        Type::DoubleArray => "double[]".to_string(),
        Type::StringArray => "string[]".to_string(),
        Type::FloatArray => "float[]".to_string(),
        Type::IntArray => "int64[]".to_string(),
        _ => "raw".to_string(),
    }
}

pub fn populate_history(obj: EnokiObject, identity: String, after: EnokiTimeStamp, before: EnokiTimeStamp) -> EnokiObject {
    let mut obj = obj;
    let entries = DATALOG.lock().get_all_entries();
    let entry_map: HashMap<String, &wpilog::log::DatalogEntryResponse> = 
        HashMap::from_iter(entries.iter().map(|entry| (entry.name.clone(), entry)));
    let mut vec_of_vec = Vec::new();
    for field in obj.get_fields() {
        let key = field.get_key().clone().prefix(identity.clone());
        if let Some(entry) = entry_map.get(&String::from(key.clone())) {
            let mut tv_vec = Vec::new();
            entry.marks.iter().for_each(|mark| {
                if mark.timestamp > after && mark.timestamp < before {
                    tv_vec.push(TimestampedEnokiValue::new(mark.timestamp, mark.value.clone().into()));
                }
            });
            vec_of_vec.push((key, tv_vec));
        }
    }
    for (key, vec) in vec_of_vec {
        obj.set_history(&key, vec);
    }
    obj
}


pub fn start_nt4_client(
    id: NetworkTableClientId,
    identity: String,
) -> Result<NetworkTableClient, EnokiError> {
    let (snd_pub, rec_pub) = channel::<EnokiObject>(255);
    let (rec_sub, snd_sub) = single_channel(HashMap::new());
    let (subscription_sender, subscription_receiver) = channel::<SubscriptionPackage>(255);
    let thread = nt4(
        id.clone(),
        identity,
        subscription_receiver,
        rec_pub,
        snd_sub,
    );
    let client = NetworkTableClient::new(id, subscription_sender, snd_pub, rec_sub, thread);

    Ok(client)
}

fn nt4(
    id: NetworkTableClientId,
    identity: String,
    mut subscriptions: Receiver<SubscriptionPackage>,
    mut input: Receiver<EnokiObject>,
    output: SingleUpdater<HashMap<String, EnokiObject>>,
) -> TokioJoinHandle<()> {
    tokio::task::Builder::new()
        .name(format!("NT4-{}", identity).as_str())
        .spawn(async move {
            let address = Ipv4Addr::new(id.ip[0], id.ip[1], id.ip[2], id.ip[3]);
            let port = id.port;

            let mut subs: HashMap<String, Subscription> = HashMap::new();
            let mut pubs: HashMap<String, PublishedTopic> = HashMap::new();

            let client = Client::try_new_w_config(
                SocketAddrV4::new(address, port),
                Config {
                    connect_timeout: 30000,
                    disconnect_retry_interval: 10000,
                    should_reconnect: Box::new(default_should_reconnect),
                    on_announce: Box::new(|topic| {
                        Box::pin(async {
                            log_result_consume(DATALOG.lock().borrow_sender().start_entry(
                                topic.name.clone(),
                                datalog_type(&topic.r#type),
                                Some("{ source: \"Enoki Network Table Client\"}".to_string()),
                            ));
                            tracing::info!("Announced {}", topic.name);
                        })
                    }),
                    on_un_announce: Box::new(|opt_topic| {
                        Box::pin(async {
                            if let Some(topic) = opt_topic {
                                log_result_consume(DATALOG
                                    .lock()
                                    .borrow_sender()
                                    .finish_entry(topic.name.clone()));
                                tracing::info!("Un-announced {}", topic.name);
                            } else {
                                tracing::info!("Un-announced unknown");
                            }
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
                identity.clone(),
            )
            .await
            .unwrap_or_else(|err| {
                tracing::error!("Failed to connect to {}:{} because {}", address, port, err);
                panic!();
            });

            let datalog_sender = DATALOG.lock().get_sender();

            let mut table = HashMap::new();

            loop {
                let start_time = std::time::Instant::now();

                let new_sub_data = subscriptions.try_recv();
                if let Ok(sub_data) = new_sub_data {
                    let topic = sub_data.topic.clone();
                    let options = sub_data.options.clone();
                    if subs.contains_key(&topic) {
                        client.unsubscribe(subs.remove(&topic).unwrap()).await.ok();
                    }
                    if !sub_data.unsubscribe {
                        let sub = client
                            .subscribe_w_options(&[topic.clone()], options)
                            .await
                            .unwrap_or_else(|err| {
                                tracing::error!("Failed to subscribe to {}:{}", address, port);
                                tracing::error!("Error: {}", err);
                                panic!();
                            });
                        tracing::info!("Subscribed to {}:{}:{}", address, port, &topic);
                        table.insert(topic.clone(), EnokiObject::new(now()));
                        subs.insert(topic, sub);
                    } else {
                        tracing::info!("Unsubscribed from {}:{}:{}", address, port, &topic);
                    }
                }

                let new_pub_data = input.try_recv();
                if let Ok(table) = new_pub_data {
                    for entry in table.get_fields() {
                        let path = String::from(entry.get_key());
                        if !pubs.contains_key(&path) {
                            let topic = client
                                .publish_topic(
                                    path.as_str(),
                                    Type::from(&entry.get_value().value),
                                    None,
                                )
                                .await
                                .unwrap();
                            pubs.insert(path.clone(), topic);
                        }
                        let topic = pubs.get(&path).unwrap();
                        client
                            .publish_value(topic, &rmpv::Value::from(&entry.get_value().value))
                            .await
                            .ok();
                        tracing::info!("Published to {}:{}:{}", address, port, path);
                    }
                }

                for (topic, sub) in subs.iter_mut() {
                    let mut new_obj_data: EnokiObject = EnokiObject::new(client.real_server_time());
                    while let Ok(msg) = sub.try_next().await {
                        let field = EnokiField::new(
                            msg.topic_name.clone().into(),
                            TimestampedEnokiValue::new(
                                client.to_real_time(msg.timestamp as u64),
                                msg.data.into(),
                            ),
                        );
                        new_obj_data.add_field(field.clone());
                        log_result_consume(datalog_sender.append_to_entry_with_timestamp(
                            EnokiKey::from(msg.topic_name).prefix(identity.clone()).into(),
                            field.get_value_owned().value.into(),
                            field.get_value().timestamp));
                    }
                    if let Some(object) = table.get_mut(topic) {
                        object.update_all(&new_obj_data)
                    }
                }
                output.update(table.clone()).unwrap_or_else(|err| {
                    tracing::error!(
                        "Failed to send to network table client {}:{}",
                        address,
                        port
                    );
                    tracing::error!("Error: {}", err);
                });

                let elapsed = start_time.elapsed();
                tokio::time::sleep(Duration::from_secs_f64(
                    (Duration::from_millis(15) - elapsed)
                        .as_secs_f64()
                        .clamp(0.0, 0.015),
                ))
                .await;
            }
        })
        .unwrap()
}

pub async fn ping_addresses(
    addresses: HashMap<String, Ipv4Addr>,
) -> Result<HashMap<String, bool>, EnokiError> {
    let mut results: HashMap<String, bool> = HashMap::new();
    for (name, address) in addresses {
        if let Ok(()) = ping::ping(IpAddr::V4(address), None, None, None, None, None) {
            results.insert(name, true);
        } else {
            results.insert(name, false);
        }
    }
    Ok(results)
}
