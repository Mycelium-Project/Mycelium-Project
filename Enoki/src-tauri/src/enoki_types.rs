use std::{collections::HashMap, fmt::{Display, self}, hash::Hash};

use serde::{
    de::Visitor,
    ser::{SerializeMap, SerializeSeq},
    Deserialize, Serialize,
};
use wpilog::log::DataLogValue;

/// Microseconds
pub type EnokiTimeStamp = u64;

pub fn now() -> EnokiTimeStamp {
    let now = std::time::SystemTime::now();
    let duration = now.duration_since(std::time::UNIX_EPOCH).unwrap();
    duration.as_micros() as EnokiTimeStamp
}

#[derive(Debug, Clone, PartialEq)]
pub enum EnokiValue {
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

impl Serialize for EnokiValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            EnokiValue::ByteArray(v) => serializer.serialize_bytes(v),
            EnokiValue::Protobuf(v) => serializer.serialize_bytes(v),
            EnokiValue::Float(v) => serializer.serialize_f64(*v),
            EnokiValue::FloatArray(v) => {
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for e in v {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
            EnokiValue::Double(v) => serializer.serialize_f64(*v),
            EnokiValue::DoubleArray(v) => {
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for e in v {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
            EnokiValue::Int(v) => serializer.serialize_i64(*v),
            EnokiValue::IntArray(v) => {
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for e in v {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
            EnokiValue::String(v) => serializer.serialize_str(v),
            EnokiValue::StringArray(v) => {
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for e in v {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
            EnokiValue::Boolean(v) => serializer.serialize_bool(*v),
            EnokiValue::BooleanArray(v) => {
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for e in v {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for EnokiValue {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<EnokiValue, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = EnokiValue;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("any valid JSON value")
            }

            #[inline]
            fn visit_bool<E>(self, value: bool) -> Result<EnokiValue, E> {
                Ok(EnokiValue::Boolean(value))
            }

            #[inline]
            fn visit_i64<E>(self, value: i64) -> Result<EnokiValue, E> {
                Ok(EnokiValue::Int(value))
            }

            #[inline]
            fn visit_u64<E>(self, value: u64) -> Result<EnokiValue, E> {
                Ok(EnokiValue::Int(value as i64))
            }

            #[inline]
            fn visit_f64<E>(self, value: f64) -> Result<EnokiValue, E> {
                Ok(EnokiValue::Double(value))
            }

            #[cfg(any(feature = "std", feature = "alloc"))]
            #[inline]
            fn visit_str<E>(self, value: &str) -> Result<Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_string(String::from(value))
            }

            #[cfg(any(feature = "std", feature = "alloc"))]
            #[inline]
            fn visit_string<E>(self, value: String) -> Result<Value, E> {
                Ok(EnokiValue::String(value))
            }

            #[inline]
            fn visit_none<E>(self) -> Result<EnokiValue, E> {
                tracing::warn!("enoki value visit none");
                Ok(EnokiValue::DoubleArray(vec![]))
            }

            #[inline]
            fn visit_some<D>(self, deserializer: D) -> Result<EnokiValue, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Deserialize::deserialize(deserializer)
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<EnokiValue, E> {
                tracing::warn!("enoki value visit unit");
                Ok(EnokiValue::DoubleArray(vec![]))
            }

            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> Result<EnokiValue, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let mut vec: Vec<EnokiValue> = Vec::new();

                while let Some(elem) = visitor.next_element()? {
                    vec.push(elem);
                }

                if vec.is_empty() {
                    Ok(EnokiValue::DoubleArray(vec![]))
                } else {
                    match vec.first().unwrap().clone() {
                        EnokiValue::Double(_) => Ok(EnokiValue::DoubleArray(
                            vec.iter().map(|v| f64::from(v)).collect(),
                        )),
                        EnokiValue::Int(_) => Ok(EnokiValue::IntArray(
                            vec.iter().map(|v| i64::from(v)).collect(),
                        )),
                        EnokiValue::Boolean(_) => Ok(EnokiValue::BooleanArray(
                            vec.iter().map(|v| bool::from(v)).collect(),
                        )),
                        EnokiValue::String(_) => Ok(EnokiValue::StringArray(
                            vec.iter().map(|v| String::from(v)).collect(),
                        )),
                        _ => {
                            tracing::warn!("enoki value visit seq");
                            Ok(EnokiValue::DoubleArray(vec![]))
                        },
                    }
                }
            }

            #[cfg(any(feature = "std", feature = "alloc"))]
            fn visit_map<V>(self, mut visitor: V) -> Result<Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                Err(serde::de::Error::custom(
                    "enoki value visit map not implemented",
                ))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

impl Display for EnokiValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnokiValue::ByteArray(v) => write!(f, "ByteArray({:?})", v),
            EnokiValue::Protobuf(v) => write!(f, "Protobuf({:?})", v),
            EnokiValue::Float(v) => write!(f, "Float({:?})", v),
            EnokiValue::FloatArray(v) => write!(f, "FloatArray({:?})", v),
            EnokiValue::Double(v) => write!(f, "Double({:?})", v),
            EnokiValue::DoubleArray(v) => write!(f, "DoubleArray({:?})", v),
            EnokiValue::Int(v) => write!(f, "Int({:?})", v),
            EnokiValue::IntArray(v) => write!(f, "IntArray({:?})", v),
            EnokiValue::String(v) => write!(f, "String({:?})", v),
            EnokiValue::StringArray(v) => write!(f, "StringArray({:?})", v),
            EnokiValue::Boolean(v) => write!(f, "Boolean({:?})", v),
            EnokiValue::BooleanArray(v) => write!(f, "BooleanArray({:?})", v),
        }
    }
}

impl EnokiValue {
    pub fn get_type(&self) -> String {
        match self {
            EnokiValue::ByteArray(_) => "ByteArray".to_string(),
            EnokiValue::Protobuf(_) => "Protobuf".to_string(),
            EnokiValue::Float(_) => "Float".to_string(),
            EnokiValue::FloatArray(_) => "FloatArray".to_string(),
            EnokiValue::Double(_) => "Double".to_string(),
            EnokiValue::DoubleArray(_) => "DoubleArray".to_string(),
            EnokiValue::Int(_) => "Int".to_string(),
            EnokiValue::IntArray(_) => "IntArray".to_string(),
            EnokiValue::String(_) => "String".to_string(),
            EnokiValue::StringArray(_) => "StringArray".to_string(),
            EnokiValue::Boolean(_) => "Boolean".to_string(),
            EnokiValue::BooleanArray(_) => "BooleanArray".to_string(),
        }
    }


    pub fn is_binary(&self) -> bool {
        match self {
            EnokiValue::ByteArray(_) => true,
            EnokiValue::Protobuf(_) => true,
            _ => false,
        }
    }

    pub fn is_numeric(&self) -> bool {
        match self {
            EnokiValue::Float(_) => true,
            EnokiValue::FloatArray(_) => true,
            EnokiValue::Double(_) => true,
            EnokiValue::DoubleArray(_) => true,
            EnokiValue::Int(_) => true,
            EnokiValue::IntArray(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            EnokiValue::String(_) => true,
            EnokiValue::StringArray(_) => true,
            _ => false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        match self {
            EnokiValue::Boolean(_) => true,
            EnokiValue::BooleanArray(_) => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            EnokiValue::ByteArray(_) => true,
            EnokiValue::Protobuf(_) => true,
            EnokiValue::FloatArray(_) => true,
            EnokiValue::DoubleArray(_) => true,
            EnokiValue::IntArray(_) => true,
            EnokiValue::StringArray(_) => true,
            EnokiValue::BooleanArray(_) => true,
            _ => false,
        }
    }

    pub fn is_single(&self) -> bool {
        match self {
            EnokiValue::Float(_) => true,
            EnokiValue::Double(_) => true,
            EnokiValue::Int(_) => true,
            EnokiValue::String(_) => true,
            EnokiValue::Boolean(_) => true,
            _ => false,
        }
    }

    pub fn get_index(&self, index: usize) -> Option<EnokiValue> {
        match self {
            EnokiValue::ByteArray(v) => v.get(index).map(|v| EnokiValue::Int(*v as i64)),
            EnokiValue::Protobuf(v) => v.get(index).map(|v| EnokiValue::Int(*v as i64)),
            EnokiValue::FloatArray(v) => v.get(index).map(|v| EnokiValue::Float(*v)),
            EnokiValue::DoubleArray(v) => v.get(index).map(|v| EnokiValue::Double(*v)),
            EnokiValue::IntArray(v) => v.get(index).map(|v| EnokiValue::Int(*v)),
            EnokiValue::StringArray(v) => v.get(index).map(|v| EnokiValue::String(v.to_owned())),
            EnokiValue::BooleanArray(v) => v.get(index).map(|v| EnokiValue::Boolean(*v)),
            _ => None,
        }
    }

    pub fn get_len(&self) -> Option<usize> {
        match self {
            EnokiValue::ByteArray(v) => Some(v.len()),
            EnokiValue::Protobuf(v) => Some(v.len()),
            EnokiValue::FloatArray(v) => Some(v.len()),
            EnokiValue::DoubleArray(v) => Some(v.len()),
            EnokiValue::IntArray(v) => Some(v.len()),
            EnokiValue::StringArray(v) => Some(v.len()),
            EnokiValue::BooleanArray(v) => Some(v.len()),
            _ => None,
        }
    }

    pub fn get<T>(&self) -> Result<T, String>
    where
        T: From<EnokiValue>,
    {
        match self {
            EnokiValue::Float(v) => Ok(T::from(EnokiValue::Float(*v))),
            EnokiValue::Double(v) => Ok(T::from(EnokiValue::Double(*v))),
            EnokiValue::Int(v) => Ok(T::from(EnokiValue::Int(*v))),
            EnokiValue::String(v) => Ok(T::from(EnokiValue::String(v.to_owned()))),
            EnokiValue::Boolean(v) => Ok(T::from(EnokiValue::Boolean(*v))),
            EnokiValue::ByteArray(v) => Ok(T::from(EnokiValue::ByteArray(v.to_owned()))),
            EnokiValue::Protobuf(v) => Ok(T::from(EnokiValue::Protobuf(v.to_owned()))),
            EnokiValue::FloatArray(v) => Ok(T::from(EnokiValue::FloatArray(v.to_owned()))),
            EnokiValue::DoubleArray(v) => Ok(T::from(EnokiValue::DoubleArray(v.to_owned()))),
            EnokiValue::IntArray(v) => Ok(T::from(EnokiValue::IntArray(v.to_owned()))),
            EnokiValue::StringArray(v) => Ok(T::from(EnokiValue::StringArray(v.to_owned()))),
            EnokiValue::BooleanArray(v) => Ok(T::from(EnokiValue::BooleanArray(v.to_owned()))),
        }
    }

    pub fn get_unwrap<T>(&self) -> T
    where
        T: From<EnokiValue>,
    {
        self.get().unwrap()
    }
}

impl From<EnokiValue> for f32 {
    fn from(m: EnokiValue) -> Self {
        match m {
            EnokiValue::Float(v) => v as f32,
            EnokiValue::Double(v) => v as f32,
            EnokiValue::Int(v) => v as f32,
            _ => panic!("Cannot convert {:?} to f32", m),
        }
    }
}

impl From<&EnokiValue> for f32 {
    fn from(m: &EnokiValue) -> Self {
        match m {
            &EnokiValue::Float(v) => v as f32,
            &EnokiValue::Double(v) => v as f32,
            &EnokiValue::Int(v) => v as f32,
            _ => panic!("Cannot convert {:?} to f32", m),
        }
    }
}

impl From<EnokiValue> for f64 {
    fn from(m: EnokiValue) -> Self {
        match m {
            EnokiValue::Double(v) => v,
            EnokiValue::Float(v) => v,
            EnokiValue::Int(v) => v as f64,
            _ => panic!("Cannot convert {:?} to f64", m),
        }
    }
}

impl From<&EnokiValue> for f64 {
    fn from(m: &EnokiValue) -> Self {
        match m {
            &EnokiValue::Double(v) => v,
            &EnokiValue::Float(v) => v,
            &EnokiValue::Int(v) => v as f64,
            _ => panic!("Cannot convert {:?} to f64", m),
        }
    }
}

impl From<EnokiValue> for i64 {
    fn from(m: EnokiValue) -> Self {
        match m {
            EnokiValue::Int(v) => v,
            EnokiValue::Float(v) => v as i64,
            EnokiValue::Double(v) => v as i64,
            _ => panic!("Cannot convert {:?} to i64", m),
        }
    }
}

impl From<&EnokiValue> for i64 {
    fn from(m: &EnokiValue) -> Self {
        match m {
            &EnokiValue::Int(v) => v,
            &EnokiValue::Float(v) => v as i64,
            &EnokiValue::Double(v) => v as i64,
            _ => panic!("Cannot convert {:?} to i64", m),
        }
    }
}

impl From<EnokiValue> for String {
    fn from(m: EnokiValue) -> Self {
        match m {
            EnokiValue::String(v) => v,
            EnokiValue::Boolean(v) => v.to_string(),
            EnokiValue::Int(v) => v.to_string(),
            EnokiValue::Float(v) => v.to_string(),
            EnokiValue::Double(v) => v.to_string(),
            _ => panic!("Cannot convert {:?} to String", m),
        }
    }
}

impl From<&EnokiValue> for String {
    fn from(m: &EnokiValue) -> Self {
        match m {
            &EnokiValue::String(ref v) => v.to_owned(),
            &EnokiValue::Boolean(v) => v.to_string(),
            &EnokiValue::Int(v) => v.to_string(),
            &EnokiValue::Float(v) => v.to_string(),
            &EnokiValue::Double(v) => v.to_string(),
            _ => panic!("Cannot convert {:?} to String", m),
        }
    }
}

impl From<EnokiValue> for bool {
    fn from(m: EnokiValue) -> Self {
        match m {
            EnokiValue::Boolean(v) => v,
            _ => panic!("Cannot convert {:?} to bool", m),
        }
    }
}

impl From<&EnokiValue> for bool {
    fn from(m: &EnokiValue) -> Self {
        match m {
            &EnokiValue::Boolean(v) => v,
            _ => panic!("Cannot convert {:?} to bool", m),
        }
    }
}

impl From<EnokiValue> for Vec<u8> {
    fn from(m: EnokiValue) -> Self {
        match m {
            EnokiValue::ByteArray(v) => v,
            _ => panic!("Cannot convert {:?} to Vec<u8>", m),
        }
    }
}

impl From<EnokiValue> for Vec<f32> {
    fn from(m: EnokiValue) -> Self {
        match m {
            EnokiValue::FloatArray(v) => v.iter().map(|v| *v as f32).collect(),
            EnokiValue::DoubleArray(v) => v.iter().map(|v| *v as f32).collect(),
            EnokiValue::IntArray(v) => v.iter().map(|v| *v as f32).collect(),
            _ => panic!("Cannot convert {:?} to Vec<f32>", m),
        }
    }
}

impl From<EnokiValue> for Vec<f64> {
    fn from(m: EnokiValue) -> Self {
        match m {
            EnokiValue::DoubleArray(v) => v,
            EnokiValue::FloatArray(v) => v.iter().map(|v| *v).collect(),
            EnokiValue::IntArray(v) => v.iter().map(|v| *v as f64).collect(),
            _ => panic!("Cannot convert {:?} to Vec<f64>", m),
        }
    }
}

impl From<EnokiValue> for Vec<i64> {
    fn from(m: EnokiValue) -> Self {
        match m {
            EnokiValue::IntArray(v) => v,
            EnokiValue::FloatArray(v) => v.iter().map(|v| *v as i64).collect(),
            EnokiValue::DoubleArray(v) => v.iter().map(|v| *v as i64).collect(),
            _ => panic!("Cannot convert {:?} to Vec<i64>", m),
        }
    }
}

impl From<EnokiValue> for Vec<String> {
    fn from(m: EnokiValue) -> Self {
        match m {
            EnokiValue::StringArray(v) => v,
            _ => panic!("Cannot convert {:?} to Vec<String>", m),
        }
    }
}

impl From<EnokiValue> for Vec<bool> {
    fn from(m: EnokiValue) -> Self {
        match m {
            EnokiValue::BooleanArray(v) => v,
            _ => panic!("Cannot convert {:?} to Vec<bool>", m),
        }
    }
}

impl From<EnokiValue> for rmpv::Value {
    fn from(m: EnokiValue) -> Self {
        match m {
            EnokiValue::Float(v) => rmpv::Value::F32(v as f32),
            EnokiValue::Double(v) => rmpv::Value::F64(v),
            EnokiValue::Int(v) => rmpv::Value::Integer(v.into()),
            EnokiValue::String(v) => rmpv::Value::String(v.into()),
            EnokiValue::Boolean(v) => rmpv::Value::Boolean(v),
            EnokiValue::ByteArray(v) => rmpv::Value::Binary(v),
            EnokiValue::Protobuf(v) => rmpv::Value::Binary(v),
            EnokiValue::FloatArray(v) => {
                rmpv::Value::Array(v.into_iter().map(|v| rmpv::Value::F32(v as f32)).collect())
            }
            EnokiValue::DoubleArray(v) => {
                rmpv::Value::Array(v.into_iter().map(|v| rmpv::Value::F64(v)).collect())
            }
            EnokiValue::IntArray(v) => rmpv::Value::Array(
                v.into_iter()
                    .map(|v| rmpv::Value::Integer(v.into()))
                    .collect(),
            ),
            EnokiValue::StringArray(v) => rmpv::Value::Array(
                v.into_iter()
                    .map(|v| rmpv::Value::String(v.into()))
                    .collect(),
            ),
            EnokiValue::BooleanArray(v) => {
                rmpv::Value::Array(v.into_iter().map(|v| rmpv::Value::Boolean(v)).collect())
            }
        }
    }
}

impl From<&EnokiValue> for rmpv::Value {
    fn from(m: &EnokiValue) -> Self {
        match m {
            EnokiValue::Float(v) => rmpv::Value::F32(*v as f32),
            EnokiValue::Double(v) => rmpv::Value::F64(*v),
            EnokiValue::Int(v) => rmpv::Value::Integer((*v).into()),
            EnokiValue::String(v) => rmpv::Value::String(v.to_owned().into()),
            EnokiValue::Boolean(v) => rmpv::Value::Boolean(*v),
            EnokiValue::ByteArray(v) => rmpv::Value::Binary(v.clone()),
            EnokiValue::Protobuf(v) => rmpv::Value::Binary(v.clone()),
            EnokiValue::FloatArray(v) => {
                rmpv::Value::Array(v.iter().map(|v| rmpv::Value::F32(*v as f32)).collect())
            }
            EnokiValue::DoubleArray(v) => {
                rmpv::Value::Array(v.iter().map(|v| rmpv::Value::F64(*v)).collect())
            }
            EnokiValue::IntArray(v) => rmpv::Value::Array(
                v.iter()
                    .map(|v| rmpv::Value::Integer((*v).into()))
                    .collect(),
            ),
            EnokiValue::StringArray(v) => rmpv::Value::Array(
                v.iter()
                    .map(|v| rmpv::Value::String(v.to_owned().into()))
                    .collect(),
            ),
            EnokiValue::BooleanArray(v) => {
                rmpv::Value::Array(v.iter().map(|v| rmpv::Value::Boolean(*v)).collect())
            }
        }
    }
}

impl From<rmpv::Value> for EnokiValue {
    fn from(v: rmpv::Value) -> Self {
        match v {
            rmpv::Value::F32(v) => EnokiValue::Float(v as f64),
            rmpv::Value::F64(v) => EnokiValue::Double(v),
            rmpv::Value::Integer(v) => EnokiValue::Int(v.as_i64().unwrap_or_default()),
            rmpv::Value::String(v) => EnokiValue::String(v.to_string().replace("\"", "")),
            rmpv::Value::Boolean(v) => EnokiValue::Boolean(v),
            rmpv::Value::Binary(v) => EnokiValue::ByteArray(v),
            rmpv::Value::Array(v) => {
                if v.len() == 0 {
                    return EnokiValue::FloatArray(Vec::new());
                }
                match v[0] {
                    rmpv::Value::F32(_) => EnokiValue::FloatArray(
                        v.into_iter()
                            .map(|v| v.as_f64().unwrap_or_default())
                            .collect(),
                    ),
                    rmpv::Value::F64(_) => EnokiValue::DoubleArray(
                        v.into_iter()
                            .map(|v| v.as_f64().unwrap_or_default())
                            .collect(),
                    ),
                    rmpv::Value::Integer(_) => EnokiValue::IntArray(
                        v.into_iter()
                            .map(|v| v.as_i64().unwrap_or_default())
                            .collect(),
                    ),
                    rmpv::Value::String(_) => EnokiValue::StringArray(
                        v.into_iter()
                            .map(|v| v.as_str().unwrap_or("").to_owned())
                            .collect(),
                    ),
                    rmpv::Value::Boolean(_) => EnokiValue::BooleanArray(
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

impl From<EnokiValue> for network_tables::v4::message_type::Type {
    fn from(m: EnokiValue) -> Self {
        match m {
            EnokiValue::Boolean(_) => network_tables::v4::message_type::Type::Boolean,
            EnokiValue::Double(_) => network_tables::v4::message_type::Type::Double,
            EnokiValue::Float(_) => network_tables::v4::message_type::Type::Float,
            EnokiValue::Int(_) => network_tables::v4::message_type::Type::Int,
            EnokiValue::String(_) => network_tables::v4::message_type::Type::String,
            EnokiValue::BooleanArray(_) => network_tables::v4::message_type::Type::BooleanArray,
            EnokiValue::DoubleArray(_) => network_tables::v4::message_type::Type::DoubleArray,
            EnokiValue::FloatArray(_) => network_tables::v4::message_type::Type::FloatArray,
            EnokiValue::IntArray(_) => network_tables::v4::message_type::Type::IntArray,
            EnokiValue::StringArray(_) => network_tables::v4::message_type::Type::StringArray,
            EnokiValue::Protobuf(_) => network_tables::v4::message_type::Type::ProtoBuf,
            EnokiValue::ByteArray(_) => network_tables::v4::message_type::Type::Raw,
        }
    }
}

impl From<&EnokiValue> for network_tables::v4::message_type::Type {
    fn from(m: &EnokiValue) -> Self {
        match m {
            EnokiValue::Boolean(_) => network_tables::v4::message_type::Type::Boolean,
            EnokiValue::Double(_) => network_tables::v4::message_type::Type::Double,
            EnokiValue::Float(_) => network_tables::v4::message_type::Type::Float,
            EnokiValue::Int(_) => network_tables::v4::message_type::Type::Int,
            EnokiValue::String(_) => network_tables::v4::message_type::Type::String,
            EnokiValue::BooleanArray(_) => network_tables::v4::message_type::Type::BooleanArray,
            EnokiValue::DoubleArray(_) => network_tables::v4::message_type::Type::DoubleArray,
            EnokiValue::FloatArray(_) => network_tables::v4::message_type::Type::FloatArray,
            EnokiValue::IntArray(_) => network_tables::v4::message_type::Type::IntArray,
            EnokiValue::StringArray(_) => network_tables::v4::message_type::Type::StringArray,
            EnokiValue::Protobuf(_) => network_tables::v4::message_type::Type::ProtoBuf,
            EnokiValue::ByteArray(_) => network_tables::v4::message_type::Type::Raw,
        }
    }
}

impl From<DataLogValue> for EnokiValue {
    fn from(m: DataLogValue) -> Self {
        match m {
            DataLogValue::Boolean(v) => EnokiValue::Boolean(v),
            DataLogValue::Double(v) => EnokiValue::Double(v),
            DataLogValue::Float(v) => EnokiValue::Float(v as f64),
            DataLogValue::Integer(v) => EnokiValue::Int(v),
            DataLogValue::String(v) => EnokiValue::String(v),
            DataLogValue::BooleanArray(v) => EnokiValue::BooleanArray(v),
            DataLogValue::DoubleArray(v) => EnokiValue::DoubleArray(v),
            DataLogValue::FloatArray(v) => {
                EnokiValue::FloatArray(v.into_iter().map(|v| v as f64).collect())
            }
            DataLogValue::IntegerArray(v) => EnokiValue::IntArray(v),
            DataLogValue::StringArray(v) => EnokiValue::StringArray(v),
            DataLogValue::Raw(v) => EnokiValue::ByteArray(v),
        }
    }
}

impl From<EnokiValue> for DataLogValue {
    fn from(m: EnokiValue) -> Self {
        match m {
            EnokiValue::Boolean(v) => DataLogValue::Boolean(v),
            EnokiValue::Double(v) => DataLogValue::Double(v),
            EnokiValue::Float(v) => DataLogValue::Float(v as f32),
            EnokiValue::Int(v) => DataLogValue::Integer(v),
            EnokiValue::String(v) => DataLogValue::String(v),
            EnokiValue::BooleanArray(v) => DataLogValue::BooleanArray(v),
            EnokiValue::DoubleArray(v) => DataLogValue::DoubleArray(v),
            EnokiValue::FloatArray(v) => {
                DataLogValue::FloatArray(v.into_iter().map(|v| v as f32).collect())
            }
            EnokiValue::IntArray(v) => DataLogValue::IntegerArray(v),
            EnokiValue::StringArray(v) => DataLogValue::StringArray(v),
            EnokiValue::ByteArray(v) => DataLogValue::Raw(v),
            _ => panic!("Cannot convert {:?} to DataLogValue", m),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TimestampedEnokiValue {
    pub value: EnokiValue,
    pub timestamp: EnokiTimeStamp,
}

impl TimestampedEnokiValue {
    pub fn new(timestamp: EnokiTimeStamp, value: EnokiValue) -> Self {
        TimestampedEnokiValue { timestamp, value }
    }
}

impl Serialize for TimestampedEnokiValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(3))?;
        match self.value.clone() {
            EnokiValue::ByteArray(v) => {
                map.serialize_entry("type", "ByteArray")?;
                map.serialize_entry("value", &v)?;
            }
            EnokiValue::Protobuf(v) => {
                map.serialize_entry("type", "Protobuf")?;
                map.serialize_entry("value", &v)?;
            }
            EnokiValue::Float(v) => {
                map.serialize_entry("type", "Float")?;
                map.serialize_entry("value", &v)?;
            }
            EnokiValue::FloatArray(v) => {
                map.serialize_entry("type", "FloatArray")?;
                map.serialize_entry("value", &v)?;
            }
            EnokiValue::Double(v) => {
                map.serialize_entry("type", "Double")?;
                map.serialize_entry("value", &v)?;
            }
            EnokiValue::DoubleArray(v) => {
                map.serialize_entry("type", "DoubleArray")?;
                map.serialize_entry("value", &v)?;
            }
            EnokiValue::Int(v) => {
                map.serialize_entry("type", "Int")?;
                map.serialize_entry("value", &v)?;
            }
            EnokiValue::IntArray(v) => {
                map.serialize_entry("type", "IntArray")?;
                map.serialize_entry("value", &v)?;
            }
            EnokiValue::String(v) => {
                map.serialize_entry("type", "String")?;
                map.serialize_entry("value", &v)?;
            }
            EnokiValue::StringArray(v) => {
                map.serialize_entry("type", "StringArray")?;
                map.serialize_entry("value", &v)?;
            }
            EnokiValue::Boolean(v) => {
                map.serialize_entry("type", "Boolean")?;
                map.serialize_entry("value", &v)?;
            }
            EnokiValue::BooleanArray(v) => {
                map.serialize_entry("type", "BooleanArray")?;
                map.serialize_entry("value", &v)?;
            }
        }
        map.serialize_entry("timestamp", &self.timestamp)?;
        map.end()
    }
}

struct TimestampedEnokiValueVisitor;
impl<'a> Visitor<'a> for TimestampedEnokiValueVisitor {
    type Value = TimestampedEnokiValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map with timestamp and value")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: serde::de::MapAccess<'a>,
    {
        let mut value = None;
        let timestamp;

        let val_type = map.next_value::<String>()?;
        match val_type.as_str() {
            "ByteArray" => {
                let v = map.next_value::<Vec<u8>>()?;
                value = Some(EnokiValue::ByteArray(v));
            }
            "Protobuf" => {
                let v = map.next_value::<Vec<u8>>()?;
                value = Some(EnokiValue::Protobuf(v));
            }
            "Float" => {
                let v = map.next_value::<f64>()?;
                value = Some(EnokiValue::Float(v));
            }
            "FloatArray" => {
                let v = map.next_value::<Vec<f64>>()?;
                value = Some(EnokiValue::FloatArray(v));
            }
            "Double" => {
                let v = map.next_value::<f64>()?;
                value = Some(EnokiValue::Double(v));
            }
            "DoubleArray" => {
                let v = map.next_value::<Vec<f64>>()?;
                value = Some(EnokiValue::DoubleArray(v));
            }
            "Int" => {
                let v = map.next_value::<i64>()?;
                value = Some(EnokiValue::Int(v));
            }
            "IntArray" => {
                let v = map.next_value::<Vec<i64>>()?;
                value = Some(EnokiValue::IntArray(v));
            }
            "String" => {
                let v = map.next_value::<String>()?;
                value = Some(EnokiValue::String(v));
            }
            "StringArray" => {
                let v = map.next_value::<Vec<String>>()?;
                value = Some(EnokiValue::StringArray(v));
            }
            "Boolean" => {
                let v = map.next_value::<bool>()?;
                value = Some(EnokiValue::Boolean(v));
            }
            "BooleanArray" => {
                let v = map.next_value::<Vec<bool>>()?;
                value = Some(EnokiValue::BooleanArray(v));
            }
            _ => {}
        };
        timestamp = Some(map.next_value::<EnokiTimeStamp>()?);

        let value = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
        let timestamp = timestamp.ok_or_else(|| serde::de::Error::missing_field("timestamp"))?;

        Ok(TimestampedEnokiValue { value, timestamp })
    }
}

impl<'a> Deserialize<'a> for TimestampedEnokiValue {
    fn deserialize<D>(deserializer: D) -> Result<TimestampedEnokiValue, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        deserializer.deserialize_map(TimestampedEnokiValueVisitor)
    }
}

impl Display for TimestampedEnokiValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}", self.value, self.timestamp)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct EnokiKey {
    path: Vec<String>,
}

impl EnokiKey {
    pub fn prefix(mut self, prefix: String) -> Self {
        self.path.insert(0, prefix);
        self
    }

    pub fn suffix(mut self, suffix: String) -> Self {
        self.path.push(suffix);
        self
    }
}

impl From<EnokiKey> for String {
    fn from(m: EnokiKey) -> Self {
        m.path.join("/")
    }
}

impl From<&EnokiKey> for String {
    fn from(m: &EnokiKey) -> Self {
        m.path.join("/")
    }
}

impl From<String> for EnokiKey {
    fn from(m: String) -> Self {
        Self {
            path: m
                .replace(":", "/")
                .split("/")
                .map(|s| s.to_string())
                .collect(),
        }
    }
}

impl From<&str> for EnokiKey {
    fn from(m: &str) -> Self {
        Self {
            path: m
                .replace(":", "/")
                .split("/")
                .map(|s| s.to_string())
                .collect(),
        }
    }
}

impl Serialize for EnokiKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        String::from(self).serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for EnokiKey {
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
        *place = Deserialize::deserialize(deserializer)?;
        Ok(())
    }
}

impl Display for EnokiKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EnokiField {
    value: TimestampedEnokiValue,
    key: EnokiKey,
}

impl Display for EnokiField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.key, self.value)
    }
}

impl EnokiField {
    pub fn new(key: EnokiKey, value: TimestampedEnokiValue) -> Self {
        Self { key, value }
    }

    pub fn get_key(&self) -> &EnokiKey {
        &self.key
    }

    pub fn get_value(&self) -> &TimestampedEnokiValue {
        &self.value
    }

    pub fn get_value_owned(&self) -> TimestampedEnokiValue {
        self.value.to_owned()
    }

    pub fn get_timestamp(&self) -> u64 {
        self.value.timestamp
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct EnokiObject {
    fields: Vec<EnokiField>,
    history: Vec<Option<Vec<TimestampedEnokiValue>>>,
    paths: HashMap<EnokiKey, usize>,
    timestamp: EnokiTimeStamp,
}

impl Display for EnokiObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Table at {}, contains:", self.timestamp)?;
        for entry in &self.fields {
            writeln!(f, "{}", String::from(entry.key.clone()))?;
        }
        Ok(())
    }
}

impl EnokiObject {
    pub fn new(timestamp: EnokiTimeStamp) -> Self {
        Self {
            timestamp,
            fields: Vec::new(),
            history: Vec::new(),
            paths: HashMap::new(),
        }
    }

    pub fn from_field(field: EnokiField) -> Self {
        let timestamp = field.get_timestamp();
        let mut obj = Self::new(timestamp);
        obj.add_field(field);
        obj
    }

    pub fn add_field(&mut self, entry: EnokiField) {
        if self.has_field(&entry.get_key()) {
            let index = self.paths.get(&entry.get_key()).unwrap();
            self.fields[*index] = entry;
        } else {
            let path = entry.get_key();
            self.paths.insert(path.to_owned(), self.fields.len());
            self.fields.push(entry);
            self.history.push(None);
        }
    }

    pub fn add_field_with_history(
        &mut self,
        entry: EnokiField,
        history: Vec<TimestampedEnokiValue>,
    ) {
        if self.has_field(&entry.get_key()) {
            let index = self.paths.get(&entry.get_key()).unwrap();
            self.fields[*index] = entry;
            self.history[*index] = Some(history);
        } else {
            let path = entry.get_key();
            self.paths.insert(path.to_owned(), self.fields.len());
            self.fields.push(entry);
            self.history.push(Some(history));
        }
    }

    pub fn set_history(&mut self, path: &EnokiKey, history: Vec<TimestampedEnokiValue>) {
        if self.has_field(path) {
            let index = self.paths.get(path).unwrap();
            self.history[*index] = Some(history);
        } else {
            let entry = EnokiField::new(path.to_owned(), history.last().unwrap().to_owned());
            self.add_field_with_history(entry, history);
        }
    }

    pub fn get_field(&self, path: &EnokiKey) -> Option<EnokiField> {
        if self.has_field(path) {
            let index = self.paths.get(path).unwrap();
            Some(self.fields[*index].clone())
        } else {
            None
        }
    }

    pub fn get_field_with_history(
        &self,
        path: &EnokiKey,
    ) -> Option<(EnokiField, Vec<TimestampedEnokiValue>)> {
        if self.has_field(path) {
            let index = self.paths.get(path).unwrap();
            let entry = self.fields[*index].clone();
            let history = self.history[*index].clone().unwrap();
            Some((entry, history))
        } else {
            None
        }
    }

    pub fn clone_without_history(&self) -> Self {
        let mut new = self.clone();
        new.history = Vec::new();
        new
    }

    pub fn get_fields(&self) -> &Vec<EnokiField> {
        &self.fields
    }

    pub fn get_timestamp(&self) -> EnokiTimeStamp {
        self.timestamp
    }

    pub fn has_field(&self, path: &EnokiKey) -> bool {
        self.paths.contains_key(&path)
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    pub fn len(&self) -> usize {
        self.fields.len()
    }

    pub fn update_fields(&mut self, other: &EnokiObject) {
        for entry in other.get_fields() {
            self.add_field(entry.to_owned());
        }
    }

    pub fn update_timestamp(&mut self, other: &EnokiObject) {
        self.timestamp = other.get_timestamp();
    }

    pub fn update_all(&mut self, other: &EnokiObject) {
        self.update_fields(other);
        self.update_timestamp(other);
    }
}
