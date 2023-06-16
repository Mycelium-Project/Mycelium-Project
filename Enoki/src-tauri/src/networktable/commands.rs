use std::net::Ipv4Addr;

use network_tables::v4::SubscriptionOptions;

use crate::{NETWORK_CLIENT_MAP, mushroom_types::{MushroomEntry, MushroomPath, MushroomTable, MushroomValue}, networktable::handler::{SubscriptionPackage, start_nt4_client}};

use super::handler::NetworkTableClientId;


#[tauri::command]
pub fn start_network_table_client(
    address: [u8; 4],
    port: u16,
    identity: String,
) -> NetworkTableClientId {
    let ip = Ipv4Addr::from(address);
    let id = NetworkTableClientId::new(ip, port, identity.clone());

    if let Some(client) = NETWORK_CLIENT_MAP.with(|map| map.borrow_mut().remove(&id)) {
        tracing::info!("Stopping network table client for {}", id);
        client.stop();
    }

    tracing::info!("Starting network table client for {}", id);
    let client = start_nt4_client(ip, port, identity).unwrap();

    NETWORK_CLIENT_MAP.with(|map| {
        map.borrow_mut().insert(id.clone(), client);
    });

    return id;
}

#[tauri::command]
pub fn does_network_table_client_exist(client_id: NetworkTableClientId) -> bool {
    NETWORK_CLIENT_MAP.with(|map| map.borrow().contains_key(&client_id))
}

#[tauri::command]
pub fn stop_network_table_client(client_id: NetworkTableClientId) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().remove(&client_id) {
            tracing::info!("Stopping network table cleint for {}", client_id);
            client.stop();
        } else {
            tracing::warn!("No network table client found for {}", client_id);
        }
    });
}

#[tauri::command]
pub fn subscribe_to_topic(
    client_id: NetworkTableClientId,
    topic: String,
    periodic: Option<f64>,
    all: Option<bool>,
    prefix: Option<bool>,
) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
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
    });
}

#[tauri::command]
pub fn set_boolean_topic(client_id: NetworkTableClientId, topic: String, value: bool) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
            let entry =
                MushroomEntry::new(MushroomValue::Boolean(value), topic.clone().into(), None);
            client.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set boolean topic {} to {}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", client_id);
        }
    });
}

#[tauri::command]
pub fn set_float_topic(client_id: NetworkTableClientId, topic: String, value: f64) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
            let entry = MushroomEntry::new(MushroomValue::Float(value), topic.clone().into(), None);
            client.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set float topic {} to {}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", client_id);
        }
    });
}

#[tauri::command]
pub fn set_double_topic(client_id: NetworkTableClientId, topic: String, value: f64) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
            let entry =
                MushroomEntry::new(MushroomValue::Double(value), topic.clone().into(), None);
            client.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set double topic {} to {}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", client_id);
        }
    });
}

#[tauri::command]
pub fn set_string_topic(client_id: NetworkTableClientId, topic: String, value: String) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
            let entry = MushroomEntry::new(
                MushroomValue::String(value.clone()),
                topic.clone().into(),
                None,
            );
            client.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set string topic {} to {}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", client_id);
        }
    });
}

#[tauri::command]
pub fn set_int_topic(client_id: NetworkTableClientId, topic: String, value: i64) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
            let entry = MushroomEntry::new(MushroomValue::Int(value), topic.clone().into(), None);
            client.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set int topic {} to {} for {}", topic, value, client_id);
        } else {
            tracing::warn!("No network table client found for {}", client_id);
        }
    });
}

#[tauri::command]
pub fn set_boolean_array_topic(client_id: NetworkTableClientId, topic: String, value: Vec<bool>) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
            let entry = MushroomEntry::new(
                MushroomValue::BooleanArray(value.clone()),
                topic.clone().into(),
                None,
            );
            client.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set boolean array topic {} to {:?}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", client_id);
        }
    });
}

#[tauri::command]
pub fn set_float_array_topic(client_id: NetworkTableClientId, topic: String, value: Vec<f64>) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
            let entry = MushroomEntry::new(
                MushroomValue::FloatArray(value.clone()),
                topic.clone().into(),
                None,
            );
            client.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set float array topic {} to {:?}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", client_id);
        }
    });
}

#[tauri::command]
pub fn set_double_array_topic(client_id: NetworkTableClientId, topic: String, value: Vec<f64>) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
            let entry = MushroomEntry::new(
                MushroomValue::DoubleArray(value.clone()),
                topic.clone().into(),
                None,
            );
            client.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set double array topic {} to {:?}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", client_id);
        }
    });
}

#[tauri::command]
pub fn set_string_array_topic(client_id: NetworkTableClientId, topic: String, value: Vec<String>) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
            let entry = MushroomEntry::new(
                MushroomValue::StringArray(value.clone()),
                topic.clone().into(),
                None,
            );
            client.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set string array topic {} to {:?}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", client_id);
        }
    });
}

#[tauri::command]
pub fn set_int_array_topic(client_id: NetworkTableClientId, topic: String, value: Vec<i64>) {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
            let entry = MushroomEntry::new(
                MushroomValue::IntArray(value.clone()),
                topic.clone().into(),
                None,
            );
            client.publish(MushroomTable::new_from_entries(0, vec![entry]));
            tracing::info!("Set int array topic {} to {:?}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", client_id);
        }
    });
}

#[tauri::command]
pub fn get_subbed_entries_values(client_id: NetworkTableClientId) -> MushroomTable {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
            tracing::info!("Getting subbed entries values for {}", client_id);
            client.poll()
        } else {
            tracing::warn!("No network table client found for {}", client_id);
            MushroomTable::new(0)
        }
    })
}

#[tauri::command]
pub fn get_subbed_entry_value(
    client_id: NetworkTableClientId,
    path: MushroomPath,
) -> Option<MushroomEntry> {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
            tracing::info!("Getting subbed entry value for {}", client_id);
            client.poll().get_entry(&path)
        } else {
            tracing::warn!("No network table client found for {}", client_id);
            None
        }
    })
}

#[tauri::command]
pub fn get_client_timestamp(client_id: NetworkTableClientId) -> f64 {
    NETWORK_CLIENT_MAP.with(|map| {
        if let Some(client) = map.borrow_mut().get_mut(&client_id) {
            tracing::info!("Getting client timestamp for {}", client_id);
            client.poll().get_timestamp() as f64 / 1000000_f64
        } else {
            tracing::warn!("No network table client found for {}", client_id);
            0_f64
        }
    })
}