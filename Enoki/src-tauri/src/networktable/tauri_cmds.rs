use std::net::Ipv4Addr;

use network_tables::v4::SubscriptionOptions;

use crate::{
    // enoki_types::{EnokiField, EnokiKey, EnokiObject, EnokiValue},
    networktable::handler::{start_nt4_client, SubscriptionPackage},
    NETWORK_CLIENT_MAP,
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
pub async fn does_network_table_client_exist(client_id: NetworkTableClientId) -> bool {
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
        client.subscribe(vec![data]);
        tracing::info!("Subscribed to topic {}", topic);
    } else {
        tracing::warn!("No network table client found for {}", client_id);
    }
}