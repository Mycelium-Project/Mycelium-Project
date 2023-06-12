


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

    pub fn get<T>(&self) -> Result<T, String> where T: From<MushroomTypes> {
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

    pub fn get_unwrap<T>(&self) -> T where T: From<MushroomTypes> {
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
            MushroomTypes::Float(v) => v as f64,
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
            MushroomTypes::FloatArray(v) => v.iter().map(|v| *v as f64).collect(),
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
            MushroomTypes::FloatArray(v) => rmpv::Value::Array(v.into_iter().map(|v| rmpv::Value::F32(v as f32)).collect()),
            MushroomTypes::DoubleArray(v) => rmpv::Value::Array(v.into_iter().map(|v| rmpv::Value::F64(v)).collect()),
            MushroomTypes::IntArray(v) => rmpv::Value::Array(v.into_iter().map(|v| rmpv::Value::Integer(v.into())).collect()),
            MushroomTypes::StringArray(v) => rmpv::Value::Array(v.into_iter().map(|v| rmpv::Value::String(v.into())).collect()),
            MushroomTypes::BooleanArray(v) => rmpv::Value::Array(v.into_iter().map(|v| rmpv::Value::Boolean(v)).collect()),
        }
    }
}

impl From<rmpv::Value> for MushroomTypes {
    fn from(v: rmpv::Value) -> Self {
        match v {
            rmpv::Value::F32(v) => MushroomTypes::Float(v as f64),
            rmpv::Value::F64(v) => MushroomTypes::Double(v),
            rmpv::Value::Integer(v) => MushroomTypes::Int(v.as_i64().unwrap_or_default()),
            rmpv::Value::String(v) => MushroomTypes::String(v.to_string()),
            rmpv::Value::Boolean(v) => MushroomTypes::Boolean(v),
            rmpv::Value::Binary(v) => MushroomTypes::ByteArray(v),
            rmpv::Value::Array(v) => {
                if v.len() == 0 {
                    return MushroomTypes::FloatArray(Vec::new());
                }
                match v[0] {
                    rmpv::Value::F32(_) => MushroomTypes::FloatArray(v.into_iter().map(|v| v.as_f64().unwrap_or_default()).collect()),
                    rmpv::Value::F64(_) => MushroomTypes::DoubleArray(v.into_iter().map(|v| v.as_f64().unwrap_or_default()).collect()),
                    rmpv::Value::Integer(_) => MushroomTypes::IntArray(v.into_iter().map(|v| v.as_i64().unwrap_or_default()).collect()),
                    rmpv::Value::String(_) => MushroomTypes::StringArray(v.into_iter().map(|v| v.as_str().unwrap_or("").to_owned()).collect()),
                    rmpv::Value::Boolean(_) => MushroomTypes::BooleanArray(v.into_iter().map(|v| v.as_bool().unwrap_or_default()).collect()),
                    _ => panic!("Cannot convert {:?} to MushroomTypes", v),
                }
            },
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

impl From<network_tables::v4::message_type::Type> for MushroomTypes {
    fn from(m: network_tables::v4::message_type::Type) -> Self {
        match m {
            network_tables::v4::message_type::Type::Boolean => MushroomTypes::Boolean(false),
            network_tables::v4::message_type::Type::Double => MushroomTypes::Double(0.0),
            network_tables::v4::message_type::Type::Float => MushroomTypes::Float(0.0),
            network_tables::v4::message_type::Type::Int => MushroomTypes::Int(0),
            network_tables::v4::message_type::Type::String => MushroomTypes::String("".to_string()),
            network_tables::v4::message_type::Type::BooleanArray => MushroomTypes::BooleanArray(vec![]),
            network_tables::v4::message_type::Type::DoubleArray => MushroomTypes::DoubleArray(vec![]),
            network_tables::v4::message_type::Type::FloatArray => MushroomTypes::FloatArray(vec![]),
            network_tables::v4::message_type::Type::IntArray => MushroomTypes::IntArray(vec![]),
            network_tables::v4::message_type::Type::StringArray => MushroomTypes::StringArray(vec![]),
            network_tables::v4::message_type::Type::ProtoBuf => MushroomTypes::Protobuf(vec![]),
            _ => MushroomTypes::ByteArray(vec![]),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MushroomEntryValue {
    value: MushroomTypes,
    path: Vec<String>,
    timestamp: Option<u64>,
}

impl MushroomEntryValue {
    pub fn new(value: MushroomTypes, path: Vec<String>, timestamp: Option<u64>) -> Self {
        Self {
            value,
            path,
            timestamp,
        }
    }

    pub fn make_path(slash_separated_path: &str) -> Vec<String> {
        slash_separated_path.split('/').map(|s| s.to_string()).collect()
    }

    pub fn get_path(&self) -> Vec<String> {
        self.path.clone()
    }

    pub fn get_path_string(&self) -> String {
        self.path.join("/")
    }

    pub fn get_value(&self) -> MushroomTypes {
        self.value.clone()
    }

    pub fn get_timestamp(&self) -> Option<u64> {
        self.timestamp.clone()
    }
}

pub type MushroomTable = Vec<MushroomEntryValue>;

