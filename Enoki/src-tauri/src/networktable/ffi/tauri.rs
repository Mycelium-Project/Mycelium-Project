use std::net::SocketAddrV4;

use crate::{networktable::handler::NetworkTableClientId, enoki_types::*};


#[tauri::command]
pub fn start_network_table_client(
    ip: SocketAddrV4,
    identity: String,
) -> NetworkTableClientId {
    super::start_network_table_client(ip, identity)
}

#[tauri::command]
pub fn is_network_table_client_stopped(client_id: NetworkTableClientId) -> bool {
    super::is_network_table_client_stopped(client_id)
}

#[tauri::command]
pub fn stop_network_table_client(client_id: NetworkTableClientId) {
    super::stop_network_table_client(client_id)
}

#[tauri::command]
pub fn subscribe_to_topic(
    client_id: NetworkTableClientId,
    topic: String,
    periodic: Option<f64>,
    all: Option<bool>,
    prefix: Option<bool>,
) {
    super::subscribe_to_topic(client_id, topic, periodic, all, prefix)
}

#[tauri::command]
pub fn unsubscribe_from_topic(
    client_id: NetworkTableClientId,
    topic: String,
) {
    super::unsubscribe_from_topic(client_id, topic)
}

#[tauri::command]
pub fn get_subbed_data(
    client_id: NetworkTableClientId,
    topic: String,
) -> Result<EnokiObject, String> {
    match super::get_subbed_data(client_id, topic) {
        Ok(data) => Ok(data),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_subbed_data_with_history(
    client_id: NetworkTableClientId,
    topic: String,
    after: EnokiTimeStamp
) -> Result<EnokiObject, String> {
    super::get_subbed_data_with_history(client_id, topic, after)
}

#[tauri::command]
pub fn set_topic_value(
    client_id: NetworkTableClientId,
    topic: String,
    value: TimestampedEnokiValue,
) {
    super::set_topic_value(client_id, topic, value)
}

#[tauri::command]
pub fn unpublish_topic(
    client_id: NetworkTableClientId,
    topic: String,
) {
    super::unpublish_topic(client_id, topic)
}