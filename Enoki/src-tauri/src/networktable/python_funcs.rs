use std::{fmt::Display, net::Ipv4Addr};

use network_tables::v4::SubscriptionOptions;
use pyo3::prelude::*;

use crate::{
    enoki_types::{EnokiField, EnokiObject, EnokiValue},
    networktable::{
        handler::{start_nt4_client, SubscriptionPackage},
        NETWORK_CLIENT_MAP,
    },
};

use super::handler::NetworkTableClientId;

#[derive(Debug, Clone)]
#[pyclass(name = "NetworkTableClient")]
pub struct PyNetworkTableClientId {
    ip: (u8, u8, u8, u8),
    port: u16,
    identity: String,
}

impl From<NetworkTableClientId> for PyNetworkTableClientId {
    fn from(id: NetworkTableClientId) -> Self {
        Self {
            ip: (id.ip[0], id.ip[1], id.ip[2], id.ip[3]),
            port: id.port,
            identity: id.identity,
        }
    }
}

impl From<PyNetworkTableClientId> for NetworkTableClientId {
    fn from(id: PyNetworkTableClientId) -> Self {
        Self {
            ip: [id.ip.0, id.ip.1, id.ip.2, id.ip.3],
            port: id.port,
            identity: id.identity,
        }
    }
}

impl Display for PyNetworkTableClientId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            Ipv4Addr::new(self.ip.0, self.ip.1, self.ip.2, self.ip.3),
            self.port,
            self.identity
        )
    }
}

#[pyfunction]
pub fn start_network_table_client(
    address: (u8, u8, u8, u8),
    port: u16,
    identity: String,
) -> PyNetworkTableClientId {
    let ip = Ipv4Addr::new(address.0, address.1, address.2, address.3);
    let id = NetworkTableClientId::new(ip, port, identity.clone());

    if let Some(client) = NETWORK_CLIENT_MAP.blocking_lock().remove(&id) {
        tracing::info!("Stopping network table client for {}", id);
        client.stop();
    }

    tracing::info!("Starting network table client for {}", id);
    let client = start_nt4_client(ip, port, identity).unwrap();

    NETWORK_CLIENT_MAP
        .blocking_lock()
        .insert(id.clone(), client);

    id.into()
}

#[pymethods]
impl PyNetworkTableClientId {
    pub fn does_network_table_client_exist(&self) -> bool {
        tracing::info!(
            "Checking if network table client exists for {}",
            NetworkTableClientId::from(self.clone())
        );
        NETWORK_CLIENT_MAP
            .blocking_lock()
            .contains_key(&self.to_owned().into())
    }

    pub fn stop_network_table_client(&self) {
        if let Some(client) = NETWORK_CLIENT_MAP
            .blocking_lock()
            .remove(&self.clone().into())
        {
            tracing::info!("Stopping network table cleint for {}", &self);
            client.stop();
        } else {
            tracing::warn!("No network table client found for {}", &self);
        }
    }

    pub fn subscribe_to_topic(
        &self,
        topic: String,
        periodic: Option<f64>,
        all: Option<bool>,
        prefix: Option<bool>,
    ) {
        if let Some(client) = NETWORK_CLIENT_MAP
            .blocking_lock()
            .get_mut(&self.clone().into())
        {
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
            tracing::warn!("No network table client found for {}", self);
        }
    }

    pub fn set_boolean_topic(&self, topic: String, value: bool) {
        if let Some(client) = NETWORK_CLIENT_MAP
            .blocking_lock()
            .get_mut(&self.clone().into())
        {
            let entry = EnokiField::new(EnokiValue::Boolean(value), topic.clone().into(), None);
            client.publish(EnokiObject::new_from_entries(0, vec![entry]));
            tracing::info!("Set boolean topic {} to {}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", self);
        }
    }

    pub fn set_float_topic(&self, topic: String, value: f64) {
        if let Some(client) = NETWORK_CLIENT_MAP
            .blocking_lock()
            .get_mut(&self.clone().into())
        {
            let entry = EnokiField::new(EnokiValue::Float(value), topic.clone().into(), None);
            client.publish(EnokiObject::new_from_entries(0, vec![entry]));
            tracing::info!("Set float topic {} to {}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", self);
        }
    }

    pub fn set_double_topic(&self, topic: String, value: f64) {
        if let Some(client) = NETWORK_CLIENT_MAP
            .blocking_lock()
            .get_mut(&self.clone().into())
        {
            let entry = EnokiField::new(EnokiValue::Double(value), topic.clone().into(), None);
            client.publish(EnokiObject::new_from_entries(0, vec![entry]));
            tracing::info!("Set double topic {} to {}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", self);
        }
    }

    pub fn set_string_topic(&self, topic: String, value: String) {
        if let Some(client) = NETWORK_CLIENT_MAP
            .blocking_lock()
            .get_mut(&self.clone().into())
        {
            let entry = EnokiField::new(
                EnokiValue::String(value.clone()),
                topic.clone().into(),
                None,
            );
            client.publish(EnokiObject::new_from_entries(0, vec![entry]));
            tracing::info!("Set string topic {} to {}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", self);
        }
    }

    pub fn set_int_topic(&self, topic: String, value: i64) {
        if let Some(client) = NETWORK_CLIENT_MAP
            .blocking_lock()
            .get_mut(&self.clone().into())
        {
            let entry = EnokiField::new(EnokiValue::Int(value), topic.clone().into(), None);
            client.publish(EnokiObject::new_from_entries(0, vec![entry]));
            tracing::info!("Set int topic {} to {}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", self);
        }
    }

    pub fn set_boolean_array_topic(&self, topic: String, value: Vec<bool>) {
        if let Some(client) = NETWORK_CLIENT_MAP
            .blocking_lock()
            .get_mut(&self.clone().into())
        {
            let entry = EnokiField::new(
                EnokiValue::BooleanArray(value.clone()),
                topic.clone().into(),
                None,
            );
            client.publish(EnokiObject::new_from_entries(0, vec![entry]));
            tracing::info!("Set boolean array topic {} to {:?}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", self);
        }
    }

    pub fn set_float_array_topic(&self, topic: String, value: Vec<f64>) {
        if let Some(client) = NETWORK_CLIENT_MAP
            .blocking_lock()
            .get_mut(&self.clone().into())
        {
            let entry = EnokiField::new(
                EnokiValue::FloatArray(value.clone()),
                topic.clone().into(),
                None,
            );
            client.publish(EnokiObject::new_from_entries(0, vec![entry]));
            tracing::info!("Set float array topic {} to {:?}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", self);
        }
    }

    pub fn set_double_array_topic(&self, topic: String, value: Vec<f64>) {
        if let Some(client) = NETWORK_CLIENT_MAP
            .blocking_lock()
            .get_mut(&self.clone().into())
        {
            let entry = EnokiField::new(
                EnokiValue::DoubleArray(value.clone()),
                topic.clone().into(),
                None,
            );
            client.publish(EnokiObject::new_from_entries(0, vec![entry]));
            tracing::info!("Set double array topic {} to {:?}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", self);
        }
    }

    pub fn set_string_array_topic(&self, topic: String, value: Vec<String>) {
        if let Some(client) = NETWORK_CLIENT_MAP
            .blocking_lock()
            .get_mut(&self.clone().into())
        {
            let entry = EnokiField::new(
                EnokiValue::StringArray(value.clone()),
                topic.clone().into(),
                None,
            );
            client.publish(EnokiObject::new_from_entries(0, vec![entry]));
            tracing::info!("Set string array topic {} to {:?}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", self);
        }
    }

    pub fn set_int_array_topic(&self, topic: String, value: Vec<i64>) {
        if let Some(client) = NETWORK_CLIENT_MAP
            .blocking_lock()
            .get_mut(&self.clone().into())
        {
            let entry = EnokiField::new(
                EnokiValue::IntArray(value.clone()),
                topic.clone().into(),
                None,
            );
            client.publish(EnokiObject::new_from_entries(0, vec![entry]));
            tracing::info!("Set int array topic {} to {:?}", topic, value);
        } else {
            tracing::warn!("No network table client found for {}", self);
        }
    }
}
