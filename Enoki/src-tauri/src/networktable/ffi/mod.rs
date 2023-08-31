
pub mod tauri;
pub mod python;

use std::net::SocketAddrV4;

use network_tables::v4::SubscriptionOptions;

use crate::{
    networktable::handler::{start_nt4_client, SubscriptionPackage},
    NETWORK_CLIENT_MAP, enoki_types::{EnokiObject, EnokiTimeStamp, now, TimestampedEnokiValue, EnokiField}, error::EnokiError,
};

use super::handler::NetworkTableClientId;

pub fn start_network_table_client(
    ip: SocketAddrV4,
    identity: String,
) -> NetworkTableClientId {
    let id = NetworkTableClientId::new(ip.ip().clone(), ip.port(), identity.clone());

    if let Some(client) = NETWORK_CLIENT_MAP.lock().remove(&id) {
        tracing::info!("Stopping network table client for {}", id);
        client.stop();
    }

    tracing::info!("Starting network table client for {}", id);
    let client = start_nt4_client(id.clone(), identity).unwrap();

    NETWORK_CLIENT_MAP.lock().insert(id.clone(), client);

    id
}

pub fn is_network_table_client_stopped(client_id: NetworkTableClientId) -> bool {
    tracing::info!("Checking if network table client exists for {}", client_id);
    NETWORK_CLIENT_MAP.lock().contains_key(&client_id)
}

pub fn stop_network_table_client(client_id: NetworkTableClientId) {
    if let Some(client) = NETWORK_CLIENT_MAP.lock().remove(&client_id) {
        tracing::info!("Stopping network table cleint for {}", client_id);
        client.stop();
    } else {
        tracing::warn!("No network table client found for {}", client_id);
    }
}

pub fn subscribe_to_topic(
    client_id: NetworkTableClientId,
    topic: String,
    periodic: Option<f64>,
    all: Option<bool>,
    prefix: Option<bool>,
) {
    if let Some(client) = NETWORK_CLIENT_MAP.lock().get_mut(&client_id) {
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

pub fn unsubscribe_from_topic(
    client_id: NetworkTableClientId,
    topic: String,
) {
    if let Some(client) = NETWORK_CLIENT_MAP.lock().get_mut(&client_id) {
        client.unsubscribe(topic.clone());
        tracing::info!("Unsubscribed from topic {}", topic);
    } else {
        tracing::warn!("No network table client found for {}", client_id);
    }
}

pub fn get_subbed_data(
    client_id: NetworkTableClientId,
    topic: String,
) -> Result<EnokiObject, EnokiError> {
    if let Some(client) = NETWORK_CLIENT_MAP.lock().get_mut(&client_id) {
        let data = client.poll(topic.clone());
        if data.is_err() {
            tracing::warn!("No data found for topic {}", &topic);
            return Err(EnokiError::NTTopicNotFound(topic));
        }
        tracing::info!("Got subbed data for topic {}", topic);
        Ok(data.unwrap())
    } else {
        tracing::warn!("No network table client found for {}", client_id);
        Err(EnokiError::NTLostConnection)
    }
}

pub fn get_subbed_data_with_history(
    client_id: NetworkTableClientId,
    topic: String,
    after: EnokiTimeStamp
) -> Result<EnokiObject, String> {
    if let Some(client) = NETWORK_CLIENT_MAP.lock().get_mut(&client_id) {
        let data = client.poll(topic.clone());
        if data.is_err() {
            tracing::warn!("No data found for topic {}", topic);
            return Err("No data found".to_string());
        }
        tracing::info!("Got subbed data for topic {}", topic);
        // Ok(populate_history(data.unwrap(), client_id.identity, after, now()))
        tracing::warn!("History not implemented");
        Err("Not implemented".to_string())
    } else {
        tracing::warn!("No network table client found for {}", client_id);
        Err("No network table client found".to_string())
    }
}

pub fn set_topic_value(
    client_id: NetworkTableClientId,
    topic: String,
    value: TimestampedEnokiValue,
) {
    if let Some(client) = NETWORK_CLIENT_MAP.lock().get_mut(&client_id) {
        tracing::info!("Set topic {} to {}", &topic, &value);
        client.publish_field(EnokiField::new(topic.into(), value));
    } else {
        tracing::warn!("No network table client found for {}", client_id);
    }
}

pub fn unpublish_topic(
    client_id: NetworkTableClientId,
    topic: String,
) {
    if let Some(_client) = NETWORK_CLIENT_MAP.lock().get_mut(&client_id) {
        tracing::info!("Unpublishing topic {}", &topic);
        // client.unpublish(topic);
        tracing::warn!("Unpublishing not implemented");
    } else {
        tracing::warn!("No network table client found for {}", client_id);
    }
}