use rsmgclient::{Node, QueryParam, Value};
use std::{collections::HashMap, error::Error, fmt::Display};

use super::{NodeTrait, NodeType, FieldTrait};

#[derive(Debug)]
pub struct Developer {
    pub id: i64,
    pub name: String,
}

pub enum Fields {
    Id,
    Name,
}

impl FieldTrait for Fields {}

impl Display for Fields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Id => write!(f, "id"),
            Self::Name => write!(f, "name"),
        }
    }
}

impl From<String> for Fields {
    fn from(value: String) -> Self {
        match value.as_str() {
            "id" => Fields::Id,
            "name" => Fields::Name,
            &_ => todo!(),
        }
    }
}

impl Into<String> for Fields {
    fn into(self) -> String {
        match self {
            Fields::Id => "id".to_string(),
            Fields::Name => "name".to_string(),
        }
    }
}

impl From<Node> for Developer {
    fn from(node: Node) -> Self {
        let properties = node.properties;
        let id = match properties.get::<String>(&Fields::Id.into()) {
            Some(Value::Int(object)) => *object,
            _ => 0,
        };
        let name = match properties.get::<String>(&Fields::Name.into()) {
            Some(Value::String(object)) => object.to_string(),
            _ => String::from(""),
        };
        Self { id, name }
    }
}

impl NodeTrait<Self> for Developer {
    fn new(id: i64, properties: &HashMap<String, Value>) -> Self {
        // default
        let mut name = String::from("");
        // using Value::String(v) is the right way, it will extract the '' from value ex 'Andy' will be Andy
        if let Value::String(v) = properties.get("name").unwrap() {
            name = v.to_string();
        }
        Self { id, name }
    }

    fn create(&self, connection: &mut rsmgclient::Connection, node: &Self) -> Result<Self, Box<dyn Error>> {
        // can't use label as aram ex $label
        let create_node = format!(r#"CREATE (n:{} {{ name: $name }}) RETURN n"#, NodeType::Developer.to_string());
        let mut params = HashMap::new();
        params.insert(Fields::Name.to_string(), QueryParam::String(node.name.clone()));
        connection
            .execute(create_node.as_str(), Some(&params))
            .unwrap();
        // must fetch before commit
        let records = connection.fetchall().unwrap();
        connection.commit().unwrap();
        let node = records[0].values[0].clone();
        match node {
            Value::Node(node) => return Ok(Self::from(node)),
            _ => return Err(format!("error can't create node type {} record", NodeType::Developer).into()),
        }
    }
}
