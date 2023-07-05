use std::collections::HashMap;

use pyo3::prelude::*;

use crate::enoki_types::{EnokiValue, TimestampedEnokiValue, EnokiField, EnokiObject};


impl IntoPy<PyObject> for EnokiValue {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            EnokiValue::Boolean(b) => b.into_py(py),
            EnokiValue::Int(i) => i.into_py(py),
            EnokiValue::Float(f) => f.into_py(py),
            EnokiValue::Double(d) => d.into_py(py),
            EnokiValue::String(s) => s.into_py(py),
            EnokiValue::BooleanArray(b) => b.into_py(py),
            EnokiValue::IntArray(i) => i.into_py(py),
            EnokiValue::FloatArray(f) => f.into_py(py),
            EnokiValue::DoubleArray(d) => d.into_py(py),
            EnokiValue::StringArray(s) => s.into_py(py),
            EnokiValue::ByteArray(b) => b.into_py(py),
            EnokiValue::Protobuf(p) => p.into_py(py),
        }
    }
}

impl FromPyObject<'_> for EnokiValue {
    fn extract(obj: &PyAny) -> PyResult<Self> {
        if let Ok(b) = obj.extract::<bool>() {
            return Ok(EnokiValue::Boolean(b));
        }
        if let Ok(i) = obj.extract::<i64>() {
            return Ok(EnokiValue::Int(i));
        }
        if let Ok(d) = obj.extract::<f64>() {
            return Ok(EnokiValue::Double(d));
        }
        if let Ok(s) = obj.extract::<String>() {
            return Ok(EnokiValue::String(s));
        }
        if let Ok(b) = obj.extract::<Vec<bool>>() {
            return Ok(EnokiValue::BooleanArray(b));
        }
        if let Ok(i) = obj.extract::<Vec<i64>>() {
            return Ok(EnokiValue::IntArray(i));
        }
        if let Ok(d) = obj.extract::<Vec<f64>>() {
            return Ok(EnokiValue::DoubleArray(d));
        }
        if let Ok(s) = obj.extract::<Vec<String>>() {
            return Ok(EnokiValue::StringArray(s));
        }
        if let Ok(b) = obj.extract::<Vec<u8>>() {
            return Ok(EnokiValue::ByteArray(b));
        }
        if let Ok(p) = obj.extract::<Vec<u8>>() {
            return Ok(EnokiValue::Protobuf(p));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Could not convert to EnokiValue",
        ))
    }
}

#[derive(Debug, Clone)]
#[pyclass(name = "TimestampedEnokiValue")]
pub struct PyTimestampedEnokiValue {
    #[pyo3(get)]
    value: EnokiValue,
    #[pyo3(get)]
    timestamp: u64,
}

#[pymethods]
impl PyTimestampedEnokiValue {
    #[new]
    fn new(value: EnokiValue, timestamp: u64) -> Self {
        PyTimestampedEnokiValue { value, timestamp }
    }
}

#[derive(Debug, Clone)]
#[pyclass(name = "EnokiField")]
pub struct PyEnokiField {
    #[pyo3(get)]
    value: EnokiValue,
    #[pyo3(get)]
    timestamp: u64,
    #[pyo3(get)]
    key: String,
}

#[pymethods]
impl PyEnokiField {
    #[new]
    fn new(value: EnokiValue, timestamp: u64, key: String) -> Self {
        PyEnokiField {
            value,
            timestamp,
            key,
        }
    }
}

#[derive(Debug, Clone)]
#[pyclass(name = "EnokiObject")]
pub struct PyEnokiObject {
    fields: Vec<PyEnokiField>,
    history: Vec<Option<Vec<PyTimestampedEnokiValue>>>,
    paths: HashMap<String, usize>,
    #[pyo3(get)]
    timestamp: u64,
}

#[pymethods]
impl PyEnokiObject {
    fn field(&self, key: String) -> PyResult<PyEnokiField> {
        if let Some(idx) = self.paths.get(&key) {
            return Ok(self.fields[*idx].clone());
        }
        Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(
            "Could not find field",
        ))
    }

    fn fields(&self) -> PyResult<Vec<PyEnokiField>> {
        Ok(self.fields.clone())
    }

    fn field_history(&self, key: String) -> PyResult<Vec<PyTimestampedEnokiValue>> {
        if let Some(idx) = self.paths.get(&key) {
            if let Some(history) = &self.history[*idx] {
                return Ok(history.clone());
            }
        }
        Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(
            "Could not find field",
        ))
    }

    fn field_keys(&self) -> PyResult<Vec<String>> {
        Ok(self.paths.keys().cloned().collect())
    }
}

impl IntoPy<PyTimestampedEnokiValue> for TimestampedEnokiValue {
    fn into_py(self, _py: Python<'_>) -> PyTimestampedEnokiValue {
        PyTimestampedEnokiValue {
            value: self.value,
            timestamp: self.timestamp,
        }
    }
}

impl FromPyObject<'_> for TimestampedEnokiValue {
    fn extract(obj: &PyAny) -> PyResult<Self> {
        let value = obj.getattr("value")?;
        let timestamp = obj.getattr("timestamp")?;
        Ok(TimestampedEnokiValue {
            value: value.extract()?,
            timestamp: timestamp.extract()?,
        })
    }
}

impl From<PyTimestampedEnokiValue> for TimestampedEnokiValue {
    fn from(py: PyTimestampedEnokiValue) -> Self {
        TimestampedEnokiValue {
            value: py.value,
            timestamp: py.timestamp,
        }
    }
}

impl From<TimestampedEnokiValue> for PyTimestampedEnokiValue {
    fn from(value: TimestampedEnokiValue) -> Self {
        PyTimestampedEnokiValue {
            value: value.value,
            timestamp: value.timestamp,
        }
    }
}

impl IntoPy<PyEnokiField> for EnokiField {
    fn into_py(self, _py: Python<'_>) -> PyEnokiField {
        PyEnokiField {
            key: self.get_key().into(),
            value: self.get_value_owned().value,
            timestamp: self.get_value().timestamp,
        }
    }
}

impl FromPyObject<'_> for EnokiField {
    fn extract(obj: &PyAny) -> PyResult<Self> {
        let key = obj.getattr("key")?;
        let value = obj.getattr("value")?;
        let timestamp = obj.getattr("timestamp")?;
        Ok(EnokiField::new(
            key.extract::<String>()?.into(),
            TimestampedEnokiValue {
                value: value.extract()?,
                timestamp: timestamp.extract()?,
            },
        ))
    }
}

impl From<PyEnokiField> for EnokiField {
    fn from(py: PyEnokiField) -> Self {
        EnokiField::new(py.key.into(), TimestampedEnokiValue {
            value: py.value,
            timestamp: py.timestamp,
        })
    }
}

impl From<EnokiField> for PyEnokiField {
    fn from(field: EnokiField) -> Self {
        PyEnokiField {
            key: field.get_key().into(),
            value: field.get_value_owned().value,
            timestamp: field.get_value().timestamp,
        }
    }
}

impl IntoPy<PyEnokiObject> for EnokiObject {
    fn into_py(self, py: Python<'_>) -> PyEnokiObject {
        let fields = self
            .get_fields()
            .iter()
            .map(|f| f.to_owned().into_py(py))
            .collect::<Vec<_>>();
        let paths = self
            .get_fields()
            .iter()
            .enumerate()
            .map(|(i, f)| (f.get_key().into(), i))
            .collect::<HashMap<_, _>>();
        let history = self
            .get_fields()
            .iter()
            .map(|f| {
                if let Some(field_and_history) = self.get_field_with_history(f.get_key()) {
                    let history = field_and_history.1;
                    Some(history.iter().map(|v| v.to_owned().into_py(py)).collect())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        PyEnokiObject {
            fields,
            paths,
            history,
            timestamp: self.get_timestamp(),
        }
    }
}

impl FromPyObject<'_> for EnokiObject {
    fn extract(obj: &PyAny) -> PyResult<Self> {
        let fields = obj.getattr("fields")?;
        let fields = fields.extract::<Vec<EnokiField>>()?;
        let timestamp = obj.getattr("timestamp")?;
        let timestamp = timestamp.extract::<u64>()?;
        let mut enoki_object = EnokiObject::new(timestamp);
        for field in fields {
            enoki_object.add_field(field);
        }
        Ok(enoki_object)
    }
}

impl From<PyEnokiObject> for EnokiObject {
    fn from(py: PyEnokiObject) -> Self {
        let mut enoki_object = EnokiObject::new(py.timestamp);
        for field in py.fields {
            enoki_object.add_field(field.into());
        }
        enoki_object
    }
}

impl From<EnokiObject> for PyEnokiObject {
    fn from(enoki_object: EnokiObject) -> Self {
        let fields = enoki_object
            .get_fields()
            .iter()
            .map(|f| f.to_owned().into())
            .collect::<Vec<_>>();
        let paths = enoki_object
            .get_fields()
            .iter()
            .enumerate()
            .map(|(i, f)| (f.get_key().into(), i))
            .collect::<HashMap<_, _>>();
        let history = enoki_object
            .get_fields()
            .iter()
            .map(|f| {
                if let Some(field_and_history) = enoki_object.get_field_with_history(f.get_key()) {
                    let history = field_and_history.1;
                    Some(history.iter().map(|v| v.to_owned().into()).collect())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        PyEnokiObject {
            fields,
            paths,
            history,
            timestamp: enoki_object.get_timestamp(),
        }
    }
}