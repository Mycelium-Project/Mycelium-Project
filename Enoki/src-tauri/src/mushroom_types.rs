use std::{collections::HashMap, hash::Hash, time::Instant, fmt::Display};

use serde::{Serialize, ser::{SerializeSeq, SerializeMap}, Deserialize};
use wpilog_rs::log::DataLogValue;

/// Microseconds
type MushroomTimeStamp = u128;

pub fn now() -> MushroomTimeStamp {
    Instant::now().elapsed().as_micros()
}

#[derive(Debug, Clone, PartialEq)]
pub enum MushroomTypes {
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

impl Serialize for MushroomTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            MushroomTypes::ByteArray(v) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "ByteArray")?;
                map.serialize_entry("value", v)?;
                map.end()
            },
            MushroomTypes::Protobuf(v) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "Protobuf")?;
                map.serialize_entry("value", v)?;
                map.end()
            },
            MushroomTypes::Float(v) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "Float")?;
                map.serialize_entry("value", v)?;
                map.end()
            },
            MushroomTypes::FloatArray(v) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "FloatArray")?;
                map.serialize_entry("value", v)?;
                map.end()
            },
            MushroomTypes::Double(v) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "Double")?;
                map.serialize_entry("value", v)?;
                map.end()
            },
            MushroomTypes::DoubleArray(v) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "DoubleArray")?;
                map.serialize_entry("value", v)?;
                map.end()
            },
            MushroomTypes::Int(v) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "Int")?;
                map.serialize_entry("value", v)?;
                map.end()
            },
            MushroomTypes::IntArray(v) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "IntArray")?;
                map.serialize_entry("value", v)?;
                map.end()
            },
            MushroomTypes::String(v) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "String")?;
                map.serialize_entry("value", v)?;
                map.end()
            },
            MushroomTypes::StringArray(v) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "StringArray")?;
                map.serialize_entry("value", v)?;
                map.end()
            },
            MushroomTypes::Boolean(v) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "Boolean")?;
                map.serialize_entry("value", v)?;
                map.end()
            },
            MushroomTypes::BooleanArray(v) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "BooleanArray")?;
                map.serialize_entry("value", v)?;
                map.end()
            },
        }

    }
}

impl Display for MushroomTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MushroomTypes::ByteArray(v) => write!(f, "ByteArray({:?})", v),
            MushroomTypes::Protobuf(v) => write!(f, "Protobuf({:?})", v),
            MushroomTypes::Float(v) => write!(f, "Float({:?})", v),
            MushroomTypes::FloatArray(v) => write!(f, "FloatArray({:?})", v),
            MushroomTypes::Double(v) => write!(f, "Double({:?})", v),
            MushroomTypes::DoubleArray(v) => write!(f, "DoubleArray({:?})", v),
            MushroomTypes::Int(v) => write!(f, "Int({:?})", v),
            MushroomTypes::IntArray(v) => write!(f, "IntArray({:?})", v),
            MushroomTypes::String(v) => write!(f, "String({:?})", v),
            MushroomTypes::StringArray(v) => write!(f, "StringArray({:?})", v),
            MushroomTypes::Boolean(v) => write!(f, "Boolean({:?})", v),
            MushroomTypes::BooleanArray(v) => write!(f, "BooleanArray({:?})", v),
        }
    }
}

impl MushroomTypes {
    pub fn is_binary(&self) -> bool {
        match self {
            MushroomTypes::ByteArray(_) => true,
            MushroomTypes::Protobuf(_) => true,
            _ => false,
        }
    }

    pub fn is_numeric(&self) -> bool {
        match self {
            MushroomTypes::Float(_) => true,
            MushroomTypes::FloatArray(_) => true,
            MushroomTypes::Double(_) => true,
            MushroomTypes::DoubleArray(_) => true,
            MushroomTypes::Int(_) => true,
            MushroomTypes::IntArray(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            MushroomTypes::String(_) => true,
            MushroomTypes::StringArray(_) => true,
            _ => false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        match self {
            MushroomTypes::Boolean(_) => true,
            MushroomTypes::BooleanArray(_) => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            MushroomTypes::ByteArray(_) => true,
            MushroomTypes::Protobuf(_) => true,
            MushroomTypes::FloatArray(_) => true,
            MushroomTypes::DoubleArray(_) => true,
            MushroomTypes::IntArray(_) => true,
            MushroomTypes::StringArray(_) => true,
            MushroomTypes::BooleanArray(_) => true,
            _ => false,
        }
    }

    pub fn is_single(&self) -> bool {
        match self {
            MushroomTypes::Float(_) => true,
            MushroomTypes::Double(_) => true,
            MushroomTypes::Int(_) => true,
            MushroomTypes::String(_) => true,
            MushroomTypes::Boolean(_) => true,
            _ => false,
        }
    }

    pub fn get_index(&self, index: usize) -> Option<MushroomTypes> {
        match self {
            MushroomTypes::ByteArray(v) => v.get(index).map(|v| MushroomTypes::Int(*v as i64)),
            MushroomTypes::Protobuf(v) => v.get(index).map(|v| MushroomTypes::Int(*v as i64)),
            MushroomTypes::FloatArray(v) => v.get(index).map(|v| MushroomTypes::Float(*v)),
            MushroomTypes::DoubleArray(v) => v.get(index).map(|v| MushroomTypes::Double(*v)),
            MushroomTypes::IntArray(v) => v.get(index).map(|v| MushroomTypes::Int(*v)),
            MushroomTypes::StringArray(v) => v.get(index).map(|v| MushroomTypes::String(v.clone())),
            MushroomTypes::BooleanArray(v) => v.get(index).map(|v| MushroomTypes::Boolean(*v)),
            _ => None,
        }
    }

    pub fn get_len(&self) -> Option<usize> {
        match self {
            MushroomTypes::ByteArray(v) => Some(v.len()),
            MushroomTypes::Protobuf(v) => Some(v.len()),
            MushroomTypes::FloatArray(v) => Some(v.len()),
            MushroomTypes::DoubleArray(v) => Some(v.len()),
            MushroomTypes::IntArray(v) => Some(v.len()),
            MushroomTypes::StringArray(v) => Some(v.len()),
            MushroomTypes::BooleanArray(v) => Some(v.len()),
            _ => None,
        }
    }

    pub fn get<T>(&self) -> Result<T, String>
    where
        T: From<MushroomTypes>,
    {
        match self {
            MushroomTypes::Float(v) => Ok(T::from(MushroomTypes::Float(*v))),
            MushroomTypes::Double(v) => Ok(T::from(MushroomTypes::Double(*v))),
            MushroomTypes::Int(v) => Ok(T::from(MushroomTypes::Int(*v))),
            MushroomTypes::String(v) => Ok(T::from(MushroomTypes::String(v.clone()))),
            MushroomTypes::Boolean(v) => Ok(T::from(MushroomTypes::Boolean(*v))),
            MushroomTypes::ByteArray(v) => Ok(T::from(MushroomTypes::ByteArray(v.clone()))),
            MushroomTypes::Protobuf(v) => Ok(T::from(MushroomTypes::Protobuf(v.clone()))),
            MushroomTypes::FloatArray(v) => Ok(T::from(MushroomTypes::FloatArray(v.clone()))),
            MushroomTypes::DoubleArray(v) => Ok(T::from(MushroomTypes::DoubleArray(v.clone()))),
            MushroomTypes::IntArray(v) => Ok(T::from(MushroomTypes::IntArray(v.clone()))),
            MushroomTypes::StringArray(v) => Ok(T::from(MushroomTypes::StringArray(v.clone()))),
            MushroomTypes::BooleanArray(v) => Ok(T::from(MushroomTypes::BooleanArray(v.clone()))),
            // _ => Err(format!("Cannot convert {:?} to {}", self, std::any::type_name::<T>())),
        }
    }

    pub fn get_unwrap<T>(&self) -> T
    where
        T: From<MushroomTypes>,
    {
        self.get().unwrap()
    }
}

impl From<MushroomTypes> for f32 {
    fn from(m: MushroomTypes) -> Self {
        match m {
            MushroomTypes::Float(v) => v as f32,
            MushroomTypes::Double(v) => v as f32,
            MushroomTypes::Int(v) => v as f32,
            _ => panic!("Cannot convert {:?} to f32", m),
        }
    }
}

impl From<MushroomTypes> for f64 {
    fn from(m: MushroomTypes) -> Self {
        match m {
            MushroomTypes::Double(v) => v,
            MushroomTypes::Float(v) => v,
            MushroomTypes::Int(v) => v as f64,
            _ => panic!("Cannot convert {:?} to f64", m),
        }
    }
}

impl From<MushroomTypes> for i64 {
    fn from(m: MushroomTypes) -> Self {
        match m {
            MushroomTypes::Int(v) => v,
            MushroomTypes::Float(v) => v as i64,
            MushroomTypes::Double(v) => v as i64,
            _ => panic!("Cannot convert {:?} to i64", m),
        }
    }
}

impl From<MushroomTypes> for String {
    fn from(m: MushroomTypes) -> Self {
        match m {
            MushroomTypes::String(v) => v,
            MushroomTypes::Boolean(v) => v.to_string(),
            MushroomTypes::Int(v) => v.to_string(),
            MushroomTypes::Float(v) => v.to_string(),
            MushroomTypes::Double(v) => v.to_string(),
            _ => panic!("Cannot convert {:?} to String", m),
        }
    }
}

impl From<MushroomTypes> for bool {
    fn from(m: MushroomTypes) -> Self {
        match m {
            MushroomTypes::Boolean(v) => v,
            _ => panic!("Cannot convert {:?} to bool", m),
        }
    }
}

impl From<MushroomTypes> for Vec<u8> {
    fn from(m: MushroomTypes) -> Self {
        match m {
            MushroomTypes::ByteArray(v) => v,
            _ => panic!("Cannot convert {:?} to Vec<u8>", m),
        }
    }
}

impl From<MushroomTypes> for Vec<f32> {
    fn from(m: MushroomTypes) -> Self {
        match m {
            MushroomTypes::FloatArray(v) => v.iter().map(|v| *v as f32).collect(),
            MushroomTypes::DoubleArray(v) => v.iter().map(|v| *v as f32).collect(),
            MushroomTypes::IntArray(v) => v.iter().map(|v| *v as f32).collect(),
            _ => panic!("Cannot convert {:?} to Vec<f32>", m),
        }
    }
}

impl From<MushroomTypes> for Vec<f64> {
    fn from(m: MushroomTypes) -> Self {
        match m {
            MushroomTypes::DoubleArray(v) => v,
            MushroomTypes::FloatArray(v) => v.iter().map(|v| *v).collect(),
            MushroomTypes::IntArray(v) => v.iter().map(|v| *v as f64).collect(),
            _ => panic!("Cannot convert {:?} to Vec<f64>", m),
        }
    }
}

impl From<MushroomTypes> for Vec<i64> {
    fn from(m: MushroomTypes) -> Self {
        match m {
            MushroomTypes::IntArray(v) => v,
            MushroomTypes::FloatArray(v) => v.iter().map(|v| *v as i64).collect(),
            MushroomTypes::DoubleArray(v) => v.iter().map(|v| *v as i64).collect(),
            _ => panic!("Cannot convert {:?} to Vec<i64>", m),
        }
    }
}

impl From<MushroomTypes> for Vec<String> {
    fn from(m: MushroomTypes) -> Self {
        match m {
            MushroomTypes::StringArray(v) => v,
            _ => panic!("Cannot convert {:?} to Vec<String>", m),
        }
    }
}

impl From<MushroomTypes> for Vec<bool> {
    fn from(m: MushroomTypes) -> Self {
        match m {
            MushroomTypes::BooleanArray(v) => v,
            _ => panic!("Cannot convert {:?} to Vec<bool>", m),
        }
    }
}

impl From<MushroomTypes> for rmpv::Value {
    fn from(m: MushroomTypes) -> Self {
        match m {
            MushroomTypes::Float(v) => rmpv::Value::F32(v as f32),
            MushroomTypes::Double(v) => rmpv::Value::F64(v),
            MushroomTypes::Int(v) => rmpv::Value::Integer(v.into()),
            MushroomTypes::String(v) => rmpv::Value::String(v.into()),
            MushroomTypes::Boolean(v) => rmpv::Value::Boolean(v),
            MushroomTypes::ByteArray(v) => rmpv::Value::Binary(v),
            MushroomTypes::Protobuf(v) => rmpv::Value::Binary(v),
            MushroomTypes::FloatArray(v) => {
                rmpv::Value::Array(v.into_iter().map(|v| rmpv::Value::F32(v as f32)).collect())
            }
            MushroomTypes::DoubleArray(v) => {
                rmpv::Value::Array(v.into_iter().map(|v| rmpv::Value::F64(v)).collect())
            }
            MushroomTypes::IntArray(v) => rmpv::Value::Array(
                v.into_iter()
                    .map(|v| rmpv::Value::Integer(v.into()))
                    .collect(),
            ),
            MushroomTypes::StringArray(v) => rmpv::Value::Array(
                v.into_iter()
                    .map(|v| rmpv::Value::String(v.into()))
                    .collect(),
            ),
            MushroomTypes::BooleanArray(v) => {
                rmpv::Value::Array(v.into_iter().map(|v| rmpv::Value::Boolean(v)).collect())
            }
        }
    }
}

impl From<rmpv::Value> for MushroomTypes {
    fn from(v: rmpv::Value) -> Self {
        match v {
            rmpv::Value::F32(v) => MushroomTypes::Float(v as f64),
            rmpv::Value::F64(v) => MushroomTypes::Double(v),
            rmpv::Value::Integer(v) => MushroomTypes::Int(v.as_i64().unwrap_or_default()),
            rmpv::Value::String(v) => MushroomTypes::String(v.to_string().replace("\"", "")),
            rmpv::Value::Boolean(v) => MushroomTypes::Boolean(v),
            rmpv::Value::Binary(v) => MushroomTypes::ByteArray(v),
            rmpv::Value::Array(v) => {
                if v.len() == 0 {
                    return MushroomTypes::FloatArray(Vec::new());
                }
                match v[0] {
                    rmpv::Value::F32(_) => MushroomTypes::FloatArray(
                        v.into_iter()
                            .map(|v| v.as_f64().unwrap_or_default())
                            .collect(),
                    ),
                    rmpv::Value::F64(_) => MushroomTypes::DoubleArray(
                        v.into_iter()
                            .map(|v| v.as_f64().unwrap_or_default())
                            .collect(),
                    ),
                    rmpv::Value::Integer(_) => MushroomTypes::IntArray(
                        v.into_iter()
                            .map(|v| v.as_i64().unwrap_or_default())
                            .collect(),
                    ),
                    rmpv::Value::String(_) => MushroomTypes::StringArray(
                        v.into_iter()
                            .map(|v| v.as_str().unwrap_or("").to_owned())
                            .collect(),
                    ),
                    rmpv::Value::Boolean(_) => MushroomTypes::BooleanArray(
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

impl From<MushroomTypes> for network_tables::v4::message_type::Type {
    fn from(m: MushroomTypes) -> Self {
        match m {
            MushroomTypes::Boolean(_) => network_tables::v4::message_type::Type::Boolean,
            MushroomTypes::Double(_) => network_tables::v4::message_type::Type::Double,
            MushroomTypes::Float(_) => network_tables::v4::message_type::Type::Float,
            MushroomTypes::Int(_) => network_tables::v4::message_type::Type::Int,
            MushroomTypes::String(_) => network_tables::v4::message_type::Type::String,
            MushroomTypes::BooleanArray(_) => network_tables::v4::message_type::Type::BooleanArray,
            MushroomTypes::DoubleArray(_) => network_tables::v4::message_type::Type::DoubleArray,
            MushroomTypes::FloatArray(_) => network_tables::v4::message_type::Type::FloatArray,
            MushroomTypes::IntArray(_) => network_tables::v4::message_type::Type::IntArray,
            MushroomTypes::StringArray(_) => network_tables::v4::message_type::Type::StringArray,
            MushroomTypes::Protobuf(_) => network_tables::v4::message_type::Type::ProtoBuf,
            MushroomTypes::ByteArray(_) => network_tables::v4::message_type::Type::Raw,
        }
    }
}

impl From<DataLogValue> for MushroomTypes {
    fn from(m: DataLogValue) -> Self {
        match m {
            DataLogValue::Boolean(v) => MushroomTypes::Boolean(v),
            DataLogValue::Double(v) => MushroomTypes::Double(v),
            DataLogValue::Float(v) => MushroomTypes::Float(v as f64),
            DataLogValue::Integer(v) => MushroomTypes::Int(v),
            DataLogValue::String(v) => MushroomTypes::String(v),
            DataLogValue::BooleanArray(v) => MushroomTypes::BooleanArray(v),
            DataLogValue::DoubleArray(v) => MushroomTypes::DoubleArray(v),
            DataLogValue::FloatArray(v) => MushroomTypes::FloatArray(
                v.into_iter().map(|v| v as f64).collect()),
            DataLogValue::IntegerArray(v) => MushroomTypes::IntArray(v),
            DataLogValue::StringArray(v) => MushroomTypes::StringArray(v),
            DataLogValue::Raw(v) => MushroomTypes::ByteArray(v),
        }
    }
}

impl From<MushroomTypes> for DataLogValue {
    fn from(m: MushroomTypes) -> Self {
        match m {
            MushroomTypes::Boolean(v) => DataLogValue::Boolean(v),
            MushroomTypes::Double(v) => DataLogValue::Double(v),
            MushroomTypes::Float(v) => DataLogValue::Float(v as f32),
            MushroomTypes::Int(v) => DataLogValue::Integer(v),
            MushroomTypes::String(v) => DataLogValue::String(v),
            MushroomTypes::BooleanArray(v) => DataLogValue::BooleanArray(v),
            MushroomTypes::DoubleArray(v) => DataLogValue::DoubleArray(v),
            MushroomTypes::FloatArray(v) => DataLogValue::FloatArray(
                v.into_iter().map(|v| v as f32).collect()),
            MushroomTypes::IntArray(v) => DataLogValue::IntegerArray(v),
            MushroomTypes::StringArray(v) => DataLogValue::StringArray(v),
            MushroomTypes::ByteArray(v) => DataLogValue::Raw(v),
            _ => panic!("Cannot convert {:?} to DataLogValue", m),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct MushroomPath {
    path: Vec<String>
}

impl From<MushroomPath> for String {
    fn from(m: MushroomPath) -> Self {
        m.path.join("/")
    }
}

impl From<String> for MushroomPath {
    fn from(m: String) -> Self {
        Self {
            path: m.split("/").map(|s| s.to_string()).collect()
        }
    }
}

impl From<&str> for MushroomPath {
    fn from(m: &str) -> Self {
        Self {
            path: m.split("/").map(|s| s.to_string()).collect()
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
    value: MushroomTypes,
    path: MushroomPath,
    timestamp: Option<f64>,
}

impl Display for MushroomEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.path, self.value)
    }
}

impl MushroomEntry {
    pub fn new(value: MushroomTypes, path: MushroomPath, timestamp: Option<f64>) -> Self {
        Self {
            value,
            path,
            timestamp,
        }
    }

    pub fn get_path(&self) -> MushroomPath {
        self.path.clone()
    }

    pub fn get_value(&self) -> MushroomTypes {
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
    entry_paths: HashMap<MushroomPath, usize>
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
        S: serde::Serializer
    {
        let mut map = serializer.serialize_seq(Some(self.entries.len()))?;
        for entry in &self.entries {
            map.serialize_element(entry)?;
        }
        map.end()
    }
}






// pub type MushroomTable = HashSet<MushroomEntry>;
