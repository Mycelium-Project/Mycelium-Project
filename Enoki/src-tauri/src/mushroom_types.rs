use std::{collections::HashMap, fmt::Display, hash::Hash, time::Instant};

use serde::{
    ser::{SerializeMap, SerializeSeq},
    Deserialize, Serialize,
};
use wpilog::log::DataLogValue;

/// Microseconds
type MushroomTimeStamp = u128;

pub fn now() -> MushroomTimeStamp {
    Instant::now().elapsed().as_micros()
}

#[derive(Debug, Clone, PartialEq)]
pub enum MushroomValue {
    ByteArray(Vec<u8>),
    Protobuf(Vec<u8>),
    Float(f64),
    FloatArray(Vec<f64>),
    Double(f64),
    DoubleArray(Vec<f64>),
    Int(i64),
    IntArray(Vec<i64>),
    String(String),
    StringArray(Vec<String>),
    Boolean(bool),
    BooleanArray(Vec<bool>),
}

impl Serialize for MushroomValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            MushroomValue::ByteArray(v) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "ByteArray")?;
                map.serialize_entry("value", v)?;
                map.end()
            }
            MushroomValue::Protobuf(v) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "Protobuf")?;
                map.serialize_entry("value", v)?;
                map.end()
            }
            MushroomValue::Float(v) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "Float")?;
                map.serialize_entry("value", v)?;
                map.end()
            }
            MushroomValue::FloatArray(v) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "FloatArray")?;
                map.serialize_entry("value", v)?;
                map.end()
            }
            MushroomValue::Double(v) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "Double")?;
                map.serialize_entry("value", v)?;
                map.end()
            }
            MushroomValue::DoubleArray(v) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "DoubleArray")?;
                map.serialize_entry("value", v)?;
                map.end()
            }
            MushroomValue::Int(v) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "Int")?;
                map.serialize_entry("value", v)?;
                map.end()
            }
            MushroomValue::IntArray(v) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "IntArray")?;
                map.serialize_entry("value", v)?;
                map.end()
            }
            MushroomValue::String(v) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "String")?;
                map.serialize_entry("value", v)?;
                map.end()
            }
            MushroomValue::StringArray(v) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "StringArray")?;
                map.serialize_entry("value", v)?;
                map.end()
            }
            MushroomValue::Boolean(v) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "Boolean")?;
                map.serialize_entry("value", v)?;
                map.end()
            }
            MushroomValue::BooleanArray(v) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "BooleanArray")?;
                map.serialize_entry("value", v)?;
                map.end()
            }
        }
    }
}

impl Display for MushroomValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MushroomValue::ByteArray(v) => write!(f, "ByteArray({:?})", v),
            MushroomValue::Protobuf(v) => write!(f, "Protobuf({:?})", v),
            MushroomValue::Float(v) => write!(f, "Float({:?})", v),
            MushroomValue::FloatArray(v) => write!(f, "FloatArray({:?})", v),
            MushroomValue::Double(v) => write!(f, "Double({:?})", v),
            MushroomValue::DoubleArray(v) => write!(f, "DoubleArray({:?})", v),
            MushroomValue::Int(v) => write!(f, "Int({:?})", v),
            MushroomValue::IntArray(v) => write!(f, "IntArray({:?})", v),
            MushroomValue::String(v) => write!(f, "String({:?})", v),
            MushroomValue::StringArray(v) => write!(f, "StringArray({:?})", v),
            MushroomValue::Boolean(v) => write!(f, "Boolean({:?})", v),
            MushroomValue::BooleanArray(v) => write!(f, "BooleanArray({:?})", v),
        }
    }
}

impl MushroomValue {
    pub fn is_binary(&self) -> bool {
        match self {
            MushroomValue::ByteArray(_) => true,
            MushroomValue::Protobuf(_) => true,
            _ => false,
        }
    }

    pub fn is_numeric(&self) -> bool {
        match self {
            MushroomValue::Float(_) => true,
            MushroomValue::FloatArray(_) => true,
            MushroomValue::Double(_) => true,
            MushroomValue::DoubleArray(_) => true,
            MushroomValue::Int(_) => true,
            MushroomValue::IntArray(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            MushroomValue::String(_) => true,
            MushroomValue::StringArray(_) => true,
            _ => false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        match self {
            MushroomValue::Boolean(_) => true,
            MushroomValue::BooleanArray(_) => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            MushroomValue::ByteArray(_) => true,
            MushroomValue::Protobuf(_) => true,
            MushroomValue::FloatArray(_) => true,
            MushroomValue::DoubleArray(_) => true,
            MushroomValue::IntArray(_) => true,
            MushroomValue::StringArray(_) => true,
            MushroomValue::BooleanArray(_) => true,
            _ => false,
        }
    }

    pub fn is_single(&self) -> bool {
        match self {
            MushroomValue::Float(_) => true,
            MushroomValue::Double(_) => true,
            MushroomValue::Int(_) => true,
            MushroomValue::String(_) => true,
            MushroomValue::Boolean(_) => true,
            _ => false,
        }
    }

    pub fn get_index(&self, index: usize) -> Option<MushroomValue> {
        match self {
            MushroomValue::ByteArray(v) => v.get(index).map(|v| MushroomValue::Int(*v as i64)),
            MushroomValue::Protobuf(v) => v.get(index).map(|v| MushroomValue::Int(*v as i64)),
            MushroomValue::FloatArray(v) => v.get(index).map(|v| MushroomValue::Float(*v)),
            MushroomValue::DoubleArray(v) => v.get(index).map(|v| MushroomValue::Double(*v)),
            MushroomValue::IntArray(v) => v.get(index).map(|v| MushroomValue::Int(*v)),
            MushroomValue::StringArray(v) => v.get(index).map(|v| MushroomValue::String(v.clone())),
            MushroomValue::BooleanArray(v) => v.get(index).map(|v| MushroomValue::Boolean(*v)),
            _ => None,
        }
    }

    pub fn get_len(&self) -> Option<usize> {
        match self {
            MushroomValue::ByteArray(v) => Some(v.len()),
            MushroomValue::Protobuf(v) => Some(v.len()),
            MushroomValue::FloatArray(v) => Some(v.len()),
            MushroomValue::DoubleArray(v) => Some(v.len()),
            MushroomValue::IntArray(v) => Some(v.len()),
            MushroomValue::StringArray(v) => Some(v.len()),
            MushroomValue::BooleanArray(v) => Some(v.len()),
            _ => None,
        }
    }

    pub fn get<T>(&self) -> Result<T, String>
    where
        T: From<MushroomValue>,
    {
        match self {
            MushroomValue::Float(v) => Ok(T::from(MushroomValue::Float(*v))),
            MushroomValue::Double(v) => Ok(T::from(MushroomValue::Double(*v))),
            MushroomValue::Int(v) => Ok(T::from(MushroomValue::Int(*v))),
            MushroomValue::String(v) => Ok(T::from(MushroomValue::String(v.clone()))),
            MushroomValue::Boolean(v) => Ok(T::from(MushroomValue::Boolean(*v))),
            MushroomValue::ByteArray(v) => Ok(T::from(MushroomValue::ByteArray(v.clone()))),
            MushroomValue::Protobuf(v) => Ok(T::from(MushroomValue::Protobuf(v.clone()))),
            MushroomValue::FloatArray(v) => Ok(T::from(MushroomValue::FloatArray(v.clone()))),
            MushroomValue::DoubleArray(v) => Ok(T::from(MushroomValue::DoubleArray(v.clone()))),
            MushroomValue::IntArray(v) => Ok(T::from(MushroomValue::IntArray(v.clone()))),
            MushroomValue::StringArray(v) => Ok(T::from(MushroomValue::StringArray(v.clone()))),
            MushroomValue::BooleanArray(v) => Ok(T::from(MushroomValue::BooleanArray(v.clone()))),
            // _ => Err(format!("Cannot convert {:?} to {}", self, std::any::type_name::<T>())),
        }
    }

    pub fn get_unwrap<T>(&self) -> T
    where
        T: From<MushroomValue>,
    {
        self.get().unwrap()
    }
}

impl From<MushroomValue> for f32 {
    fn from(m: MushroomValue) -> Self {
        match m {
            MushroomValue::Float(v) => v as f32,
            MushroomValue::Double(v) => v as f32,
            MushroomValue::Int(v) => v as f32,
            _ => panic!("Cannot convert {:?} to f32", m),
        }
    }
}

impl From<MushroomValue> for f64 {
    fn from(m: MushroomValue) -> Self {
        match m {
            MushroomValue::Double(v) => v,
            MushroomValue::Float(v) => v,
            MushroomValue::Int(v) => v as f64,
            _ => panic!("Cannot convert {:?} to f64", m),
        }
    }
}

impl From<MushroomValue> for i64 {
    fn from(m: MushroomValue) -> Self {
        match m {
            MushroomValue::Int(v) => v,
            MushroomValue::Float(v) => v as i64,
            MushroomValue::Double(v) => v as i64,
            _ => panic!("Cannot convert {:?} to i64", m),
        }
    }
}

impl From<MushroomValue> for String {
    fn from(m: MushroomValue) -> Self {
        match m {
            MushroomValue::String(v) => v,
            MushroomValue::Boolean(v) => v.to_string(),
            MushroomValue::Int(v) => v.to_string(),
            MushroomValue::Float(v) => v.to_string(),
            MushroomValue::Double(v) => v.to_string(),
            _ => panic!("Cannot convert {:?} to String", m),
        }
    }
}

impl From<MushroomValue> for bool {
    fn from(m: MushroomValue) -> Self {
        match m {
            MushroomValue::Boolean(v) => v,
            _ => panic!("Cannot convert {:?} to bool", m),
        }
    }
}

impl From<MushroomValue> for Vec<u8> {
    fn from(m: MushroomValue) -> Self {
        match m {
            MushroomValue::ByteArray(v) => v,
            _ => panic!("Cannot convert {:?} to Vec<u8>", m),
        }
    }
}

impl From<MushroomValue> for Vec<f32> {
    fn from(m: MushroomValue) -> Self {
        match m {
            MushroomValue::FloatArray(v) => v.iter().map(|v| *v as f32).collect(),
            MushroomValue::DoubleArray(v) => v.iter().map(|v| *v as f32).collect(),
            MushroomValue::IntArray(v) => v.iter().map(|v| *v as f32).collect(),
            _ => panic!("Cannot convert {:?} to Vec<f32>", m),
        }
    }
}

impl From<MushroomValue> for Vec<f64> {
    fn from(m: MushroomValue) -> Self {
        match m {
            MushroomValue::DoubleArray(v) => v,
            MushroomValue::FloatArray(v) => v.iter().map(|v| *v).collect(),
            MushroomValue::IntArray(v) => v.iter().map(|v| *v as f64).collect(),
            _ => panic!("Cannot convert {:?} to Vec<f64>", m),
        }
    }
}

impl From<MushroomValue> for Vec<i64> {
    fn from(m: MushroomValue) -> Self {
        match m {
            MushroomValue::IntArray(v) => v,
            MushroomValue::FloatArray(v) => v.iter().map(|v| *v as i64).collect(),
            MushroomValue::DoubleArray(v) => v.iter().map(|v| *v as i64).collect(),
            _ => panic!("Cannot convert {:?} to Vec<i64>", m),
        }
    }
}

impl From<MushroomValue> for Vec<String> {
    fn from(m: MushroomValue) -> Self {
        match m {
            MushroomValue::StringArray(v) => v,
            _ => panic!("Cannot convert {:?} to Vec<String>", m),
        }
    }
}

impl From<MushroomValue> for Vec<bool> {
    fn from(m: MushroomValue) -> Self {
        match m {
            MushroomValue::BooleanArray(v) => v,
            _ => panic!("Cannot convert {:?} to Vec<bool>", m),
        }
    }
}

impl From<MushroomValue> for rmpv::Value {
    fn from(m: MushroomValue) -> Self {
        match m {
            MushroomValue::Float(v) => rmpv::Value::F32(v as f32),
            MushroomValue::Double(v) => rmpv::Value::F64(v),
            MushroomValue::Int(v) => rmpv::Value::Integer(v.into()),
            MushroomValue::String(v) => rmpv::Value::String(v.into()),
            MushroomValue::Boolean(v) => rmpv::Value::Boolean(v),
            MushroomValue::ByteArray(v) => rmpv::Value::Binary(v),
            MushroomValue::Protobuf(v) => rmpv::Value::Binary(v),
            MushroomValue::FloatArray(v) => {
                rmpv::Value::Array(v.into_iter().map(|v| rmpv::Value::F32(v as f32)).collect())
            }
            MushroomValue::DoubleArray(v) => {
                rmpv::Value::Array(v.into_iter().map(|v| rmpv::Value::F64(v)).collect())
            }
            MushroomValue::IntArray(v) => rmpv::Value::Array(
                v.into_iter()
                    .map(|v| rmpv::Value::Integer(v.into()))
                    .collect(),
            ),
            MushroomValue::StringArray(v) => rmpv::Value::Array(
                v.into_iter()
                    .map(|v| rmpv::Value::String(v.into()))
                    .collect(),
            ),
            MushroomValue::BooleanArray(v) => {
                rmpv::Value::Array(v.into_iter().map(|v| rmpv::Value::Boolean(v)).collect())
            }
        }
    }
}

impl From<rmpv::Value> for MushroomValue {
    fn from(v: rmpv::Value) -> Self {
        match v {
            rmpv::Value::F32(v) => MushroomValue::Float(v as f64),
            rmpv::Value::F64(v) => MushroomValue::Double(v),
            rmpv::Value::Integer(v) => MushroomValue::Int(v.as_i64().unwrap_or_default()),
            rmpv::Value::String(v) => MushroomValue::String(v.to_string().replace("\"", "")),
            rmpv::Value::Boolean(v) => MushroomValue::Boolean(v),
            rmpv::Value::Binary(v) => MushroomValue::ByteArray(v),
            rmpv::Value::Array(v) => {
                if v.len() == 0 {
                    return MushroomValue::FloatArray(Vec::new());
                }
                match v[0] {
                    rmpv::Value::F32(_) => MushroomValue::FloatArray(
                        v.into_iter()
                            .map(|v| v.as_f64().unwrap_or_default())
                            .collect(),
                    ),
                    rmpv::Value::F64(_) => MushroomValue::DoubleArray(
                        v.into_iter()
                            .map(|v| v.as_f64().unwrap_or_default())
                            .collect(),
                    ),
                    rmpv::Value::Integer(_) => MushroomValue::IntArray(
                        v.into_iter()
                            .map(|v| v.as_i64().unwrap_or_default())
                            .collect(),
                    ),
                    rmpv::Value::String(_) => MushroomValue::StringArray(
                        v.into_iter()
                            .map(|v| v.as_str().unwrap_or("").to_owned())
                            .collect(),
                    ),
                    rmpv::Value::Boolean(_) => MushroomValue::BooleanArray(
                        v.into_iter()
                            .map(|v| v.as_bool().unwrap_or_default())
                            .collect(),
                    ),
                    _ => panic!("Cannot convert {:?} to MushroomTypes", v),
                }
            }
            _ => panic!("Cannot convert {:?} to MushroomTypes", v),
        }
    }
}

impl From<MushroomValue> for network_tables::v4::message_type::Type {
    fn from(m: MushroomValue) -> Self {
        match m {
            MushroomValue::Boolean(_) => network_tables::v4::message_type::Type::Boolean,
            MushroomValue::Double(_) => network_tables::v4::message_type::Type::Double,
            MushroomValue::Float(_) => network_tables::v4::message_type::Type::Float,
            MushroomValue::Int(_) => network_tables::v4::message_type::Type::Int,
            MushroomValue::String(_) => network_tables::v4::message_type::Type::String,
            MushroomValue::BooleanArray(_) => network_tables::v4::message_type::Type::BooleanArray,
            MushroomValue::DoubleArray(_) => network_tables::v4::message_type::Type::DoubleArray,
            MushroomValue::FloatArray(_) => network_tables::v4::message_type::Type::FloatArray,
            MushroomValue::IntArray(_) => network_tables::v4::message_type::Type::IntArray,
            MushroomValue::StringArray(_) => network_tables::v4::message_type::Type::StringArray,
            MushroomValue::Protobuf(_) => network_tables::v4::message_type::Type::ProtoBuf,
            MushroomValue::ByteArray(_) => network_tables::v4::message_type::Type::Raw,
        }
    }
}

impl From<DataLogValue> for MushroomValue {
    fn from(m: DataLogValue) -> Self {
        match m {
            DataLogValue::Boolean(v) => MushroomValue::Boolean(v),
            DataLogValue::Double(v) => MushroomValue::Double(v),
            DataLogValue::Float(v) => MushroomValue::Float(v as f64),
            DataLogValue::Integer(v) => MushroomValue::Int(v),
            DataLogValue::String(v) => MushroomValue::String(v),
            DataLogValue::BooleanArray(v) => MushroomValue::BooleanArray(v),
            DataLogValue::DoubleArray(v) => MushroomValue::DoubleArray(v),
            DataLogValue::FloatArray(v) => {
                MushroomValue::FloatArray(v.into_iter().map(|v| v as f64).collect())
            }
            DataLogValue::IntegerArray(v) => MushroomValue::IntArray(v),
            DataLogValue::StringArray(v) => MushroomValue::StringArray(v),
            DataLogValue::Raw(v) => MushroomValue::ByteArray(v),
        }
    }
}

impl From<MushroomValue> for DataLogValue {
    fn from(m: MushroomValue) -> Self {
        match m {
            MushroomValue::Boolean(v) => DataLogValue::Boolean(v),
            MushroomValue::Double(v) => DataLogValue::Double(v),
            MushroomValue::Float(v) => DataLogValue::Float(v as f32),
            MushroomValue::Int(v) => DataLogValue::Integer(v),
            MushroomValue::String(v) => DataLogValue::String(v),
            MushroomValue::BooleanArray(v) => DataLogValue::BooleanArray(v),
            MushroomValue::DoubleArray(v) => DataLogValue::DoubleArray(v),
            MushroomValue::FloatArray(v) => {
                DataLogValue::FloatArray(v.into_iter().map(|v| v as f32).collect())
            }
            MushroomValue::IntArray(v) => DataLogValue::IntegerArray(v),
            MushroomValue::StringArray(v) => DataLogValue::StringArray(v),
            MushroomValue::ByteArray(v) => DataLogValue::Raw(v),
            _ => panic!("Cannot convert {:?} to DataLogValue", m),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct MushroomPath {
    path: Vec<String>,
}

impl From<MushroomPath> for String {
    fn from(m: MushroomPath) -> Self {
        m.path.join("/")
    }
}

impl From<String> for MushroomPath {
    fn from(m: String) -> Self {
        Self {
            path: m.split("/").map(|s| s.to_string()).collect(),
        }
    }
}

impl From<&str> for MushroomPath {
    fn from(m: &str) -> Self {
        Self {
            path: m.split("/").map(|s| s.to_string()).collect(),
        }
    }
}

impl Serialize for MushroomPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        String::from(self.clone()).serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for MushroomPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        String::deserialize(deserializer).map(|s| s.into())
    }

    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        // Default implementation just delegates to `deserialize` impl.
        *place = Deserialize::deserialize(deserializer)?;
        Ok(())
    }
}

impl Display for MushroomPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self.clone()))
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MushroomEntry {
    value: MushroomValue,
    path: MushroomPath,
    timestamp: Option<f64>,
}

impl Display for MushroomEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.path, self.value)
    }
}

impl MushroomEntry {
    pub fn new(value: MushroomValue, path: MushroomPath, timestamp: Option<f64>) -> Self {
        Self {
            value,
            path,
            timestamp,
        }
    }

    pub fn get_path(&self) -> MushroomPath {
        self.path.clone()
    }

    pub fn get_value(&self) -> MushroomValue {
        self.value.clone()
    }

    pub fn get_timestamp(&self) -> Option<f64> {
        self.timestamp.clone()
    }
}

#[derive(Clone, Debug)]
pub struct MushroomTable {
    timestamp: MushroomTimeStamp,
    //could use a set but this is easier
    entries: Vec<MushroomEntry>,
    entry_paths: HashMap<MushroomPath, usize>,
}

impl Display for MushroomTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Table at {}", self.timestamp)?;
        for entry in &self.entries {
            writeln!(f, "{}", entry)?;
        }
        Ok(())
    }
}

impl MushroomTable {
    pub fn new(timestamp: MushroomTimeStamp) -> Self {
        Self {
            timestamp,
            entries: Vec::new(),
            entry_paths: HashMap::new(),
        }
    }

    pub fn new_from_entries(timestamp: MushroomTimeStamp, entries: Vec<MushroomEntry>) -> Self {
        let mut entry_paths = HashMap::new();
        for (i, entry) in entries.iter().enumerate() {
            entry_paths.insert(entry.get_path().into(), i);
        }
        Self {
            timestamp,
            entries,
            entry_paths,
        }
    }

    pub fn add_entry(&mut self, entry: MushroomEntry) {
        if self.has_entry(&entry.get_path()) {
            let index = self.entry_paths.get(&entry.get_path()).unwrap();
            self.entries[*index] = entry;
        } else {
            let path = entry.get_path();
            self.entries.push(entry);
            self.entry_paths.insert(path, self.entries.len() - 1);
        }
    }

    pub fn get_entry(&self, path: &MushroomPath) -> Option<MushroomEntry> {
        if self.has_entry(path) {
            let index = self.entry_paths.get(path).unwrap();
            Some(self.entries[*index].clone())
        } else {
            None
        }
    }

    pub fn get_entries(&self) -> &Vec<MushroomEntry> {
        &self.entries
    }

    pub fn get_timestamp(&self) -> MushroomTimeStamp {
        self.timestamp
    }

    pub fn has_entry(&self, path: &MushroomPath) -> bool {
        self.entry_paths.contains_key(&path)
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn update_entries(&mut self, other: &MushroomTable) {
        for entry in other.get_entries() {
            self.add_entry(entry.clone());
        }
    }

    pub fn update_timestamp(&mut self, other: &MushroomTable) {
        self.timestamp = other.get_timestamp();
    }

    pub fn update_all(&mut self, other: &MushroomTable) {
        self.update_entries(other);
        self.update_timestamp(other);
    }
}

impl Serialize for MushroomTable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_seq(Some(self.entries.len()))?;
        for entry in &self.entries {
            map.serialize_element(entry)?;
        }
        map.end()
    }
}

// pub type MushroomTable = HashSet<MushroomEntry>;
