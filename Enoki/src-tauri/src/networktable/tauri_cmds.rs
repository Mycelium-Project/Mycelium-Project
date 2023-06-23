use std::net::Ipv4Addr;

use network_tables::v4::SubscriptionOptions;

use crate::{
    networktable::handler::{start_nt4_client, SubscriptionPackage, populate_history},
    NETWORK_CLIENT_MAP, enoki_types::{EnokiObject, EnokiTimeStamp, now, TimestampedEnokiValue, EnokiField},
};

use super::handler::NetworkTableClientId;

#[tauri::command]
pub async fn start_network_table_client(
    address: [u8; 4],
    port: u16,
    identity: String,
) -> NetworkTableClientId {
    let ip = Ipv4Addr::from(address);
    let id = NetworkTableClientId::new(ip, port, identity.clone());

    if let Some(client) = NETWORK_CLIENT_MAP.lock().await.remove(&id) {
        tracing::info!("Stopping network table client for {}", id);
        client.stop();
    }

    tracing::info!("Starting network table client for {}", id);
    let client = start_nt4_client(ip, port, identity).unwrap();

    NETWORK_CLIENT_MAP.lock().await.insert(id.clone(), client);

    id
}

#[tauri::command]
pub async fn is_network_table_client_stopped(client_id: NetworkTableClientId) -> bool {
    tracing::info!("Checking if network table client exists for {}", client_id);
    NETWORK_CLIENT_MAP.lock().await.contains_key(&client_id)
}

#[tauri::command]
pub async fn stop_network_table_client(client_id: NetworkTableClientId) {
    if let Some(client) = NETWORK_CLIENT_MAP.lock().await.remove(&client_id) {
        tracing::info!("Stopping network table cleint for {}", client_id);
        client.stop();
    } else {
        tracing::warn!("No network table client found for {}", client_id);
    }
}

#[tauri::command]
pub async fn subscribe_to_topic(
    client_id: NetworkTableClientId,
    topic: String,
    periodic: Option<f64>,
    all: Option<bool>,
    prefix: Option<bool>,
) {
    if let Some(client) = NETWORK_CLIENT_MAP.lock().await.get_mut(&client_id) {
        let data = SubscriptionPackage::new(
            topic.clone(),
            SubscriptionOptions {
                all,
                prefix,
                periodic,
                ..Default::default()
            },
        );
        client.subscribe(data);
        tracing::info!("Subscribed to topic {}", topic);
    } else {
        tracing::warn!("No network table client found for {}", client_id);
    }
}

#[tauri::command]
pub async fn unsubscribe_from_topic(
    client_id: NetworkTableClientId,
    topic: String,
) {
    if let Some(client) = NETWORK_CLIENT_MAP.lock().await.get_mut(&client_id) {
        client.unsubscribe(topic.clone());
        tracing::info!("Unsubscribed from topic {}", topic);
    } else {
        tracing::warn!("No network table client found for {}", client_id);
    }
}

#[tauri::command]
pub async fn get_subbed_data(
    client_id: NetworkTableClientId,
    topic: String,
) -> Result<EnokiObject, String> {
    if let Some(client) = NETWORK_CLIENT_MAP.lock().await.get_mut(&client_id) {
        let data = client.poll(topic.clone());
        if data.is_err() {
            tracing::warn!("No data found for topic {}", topic);
            return Err("No data found".to_string());
        }
        tracing::info!("Got subbed data for topic {}", topic);
        Ok(data.unwrap())
    } else {
        tracing::warn!("No network table client found for {}", client_id);
        Err("No network table client found".to_string())
    }
}

#[tauri::command]
pub async fn get_subbed_data_with_history(
    client_id: NetworkTableClientId,
    topic: String,
    after: EnokiTimeStamp
) -> Result<EnokiObject, String> {
    if let Some(client) = NETWORK_CLIENT_MAP.lock().await.get_mut(&client_id) {
        let data = client.poll(topic.clone());
        if data.is_err() {
            tracing::warn!("No data found for topic {}", topic);
            return Err("No data found".to_string());
        }
        tracing::info!("Got subbed data for topic {}", topic);
        Ok(populate_history(data.unwrap(), client_id.identity, after, now()).await)
    } else {
        tracing::warn!("No network table client found for {}", client_id);
        Err("No network table client found".to_string())
    }
}

#[tauri::command]
pub async fn set_topic_value(
    client_id: NetworkTableClientId,
    topic: String,
    value: TimestampedEnokiValue,
) {
    if let Some(client) = NETWORK_CLIENT_MAP.lock().await.get_mut(&client_id) {
        tracing::info!("Set topic {} to {}", &topic, &value);
        client.publish_field(EnokiField::new(topic.into(), value));
    } else {
        tracing::warn!("No network table client found for {}", client_id);
    }
}

#[tauri::command]
pub async fn unpublish_topic(
    client_id: NetworkTableClientId,
    topic: String,
) {
    if let Some(_client) = NETWORK_CLIENT_MAP.lock().await.get_mut(&client_id) {
        tracing::info!("Unpublishing topic {}", &topic);
        // client.unpublish(topic);
        tracing::warn!("Unpublishing not implemented");
    } else {
        tracing::warn!("No network table client found for {}", client_id);
    }
}