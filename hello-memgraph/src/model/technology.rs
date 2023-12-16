use rsmgclient::{Value, QueryParam};
use std::{collections::HashMap, error::Error, any::type_name, fmt::Display};

use super::NodeTrait;

#[derive(Debug)]
pub struct Technology {
    pub id: i64,
    pub name: String,
    pub description: String,
}

pub enum Fields {
  Id,
  Name,
  Description,
}

impl Display for Fields {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match *self {
          Self::Id => write!(f, "id"),
          Self::Name => write!(f, "name"),
          Self::Description => write!(f, "description"),
      }
  }
}

impl From<String> for Fields {
  fn from(value: String) -> Self {
      match value.as_str() {
          "id" => Fields::Id,
          "name" => Fields::Name,
          "description" => Fields::Description,
          &_ => todo!(),
      }
  }
}

impl Into<String> for Fields {
  fn into(self) -> String {
      match self {
          Fields::Id => "id".to_string(),
          Fields::Name => "name".to_string(),
          Fields::Description => "description".to_string(),
      }
  }
}

impl NodeTrait<Self> for Technology {
    fn new(id: i64, properties: &HashMap<String, Value>) -> Self {
        // default
        let mut name = String::from("");
        let mut description = String::from("");
        // using Value::String(v) is the right way, it will extract the '' from value ex 'Andy' will be Andy
        if let Value::String(v) = properties.get("name").unwrap() {
            name = v.to_string();
        }
        if let Value::String(v) = properties.get("description").unwrap() {
            description = v.to_string();
        }
        Self {
            id,
            name,
            description,
        }
    }

    fn create(&self, connection: &mut rsmgclient::Connection, node: &Self) -> Result<Self, Box<dyn Error>> {
        let create_node = "CREATE (n:$label {name: $name, description: $description}) RETURN n";
        let mut params = HashMap::new();
        params.insert(
            "label".to_string(),
            QueryParam::String(type_name::<Self>().to_string()),
        );
        params.insert(Fields::Name.into(), QueryParam::String(node.name.clone()));
        params.insert(Fields::Description.into(), QueryParam::String(node.description.clone()));
        connection.execute(create_node, Some(&params)).unwrap();
        connection.commit().unwrap();
        Ok(Self {
            id: 1,
            name: "".to_string(),
            description: "".to_string(),
        })
    }
}
