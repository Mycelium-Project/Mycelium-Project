
use std::net::SocketAddrV4;

use pyo3::prelude::*;

use crate::{networktable::handler::NetworkTableClientId, enoki_types::*, python_helpers::py_enoki_types::PyEnokiObject};

#[derive(Debug, Clone)]
#[pyclass(name = "NetworkTableClientId")]
pub struct PyNetworkTableClientId {
    #[pyo3(get)]
    ip: (u8, u8, u8, u8),
    #[pyo3(get)]
    port: u16,
    #[pyo3(get)]
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

#[derive(Debug, Clone)]
#[pyclass(name = "NetworkTablePubbedTopic")]
pub struct PyNetworkTablePubbedTopic {
    topic: String,
    r#type: String,
    client_id: PyNetworkTableClientId,
}

#[pymethods]
impl PyNetworkTablePubbedTopic {
    #[new]
    fn new(topic: String, type_name: String, client_id: PyNetworkTableClientId) -> Self {
        Self {
            topic,
            r#type: type_name,
            client_id,
        }
    }


    fn topic_name(&self) -> String {
        self.topic.clone()
    }

    fn topic_type(&self) -> String {
        self.r#type.clone()
    }

    fn set_value(&self, value: EnokiValue, timestamp: Option<u64>) -> PyResult<()> {
        if value.get_type() != self.r#type {
            tracing::error!("Value type mismatch");
            return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                "Value type mismatch: expected {}, got {}",
                self.r#type,
                value.get_type()
            )));
        }
        let timestamp = timestamp.unwrap_or_else(now);
        super::set_topic_value(
            self.client_id.clone().into(),
            self.topic.clone(),
            TimestampedEnokiValue::new(timestamp, value),
        );
        Ok(())
    }

    fn unpublish(&self) {
        super::unpublish_topic(
            self.client_id.clone().into(), self.topic.clone())
    }
}

#[pyclass(name = "NetworkTableSubscription")]
pub struct PyNetworkTableSubscription {
    topic: String,
    client_id: PyNetworkTableClientId,
}

#[pymethods]
impl PyNetworkTableSubscription {
    #[new]
    fn new(topic: String, client_id: PyNetworkTableClientId) -> Self {
        Self { topic, client_id }
    }


    fn get_subbed_data(&self) -> PyResult<PyEnokiObject> {
        match super::get_subbed_data(self.client_id.clone().into(), self.topic.clone()) {
            Ok(data) => Ok(data.into()),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Error getting subbed data: {}",
                e
            ))),
        }
    }

    fn get_subbed_data_with_history(&self, after: u64) -> PyResult<PyEnokiObject> {
        match super::get_subbed_data_with_history(self.client_id.clone().into(), self.topic.clone(), after) {
            Ok(data) => Ok(data.into()),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Error getting subbed data: {}",
                e
            ))),
        }
    }
}

#[pyfunction]
pub fn start_network_table_client(
    address: (u8, u8, u8, u8),
    port: u16,
    identity: String,
) -> PyNetworkTableClientId {
    let ip = SocketAddrV4::new(
        std::net::Ipv4Addr::new(address.0, address.1, address.2, address.3),
        port,
    );
    super::start_network_table_client(ip, identity).into()
}