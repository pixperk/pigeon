use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct Schema{
    pub service : String,
    pub version : String,
    pub rpcs    : Vec<Rpc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Rpc{
    pub name : String,
    pub method : String,
    pub path : String,
    pub request : FieldMap,
    pub response : FieldMap
}


pub type FieldMap = HashMap<String, FieldDef>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FieldDef {
    Simple(String),   // string, integer, etc.
    Detailed(Field),  // full struct
}

impl FieldDef {
    pub fn into_field(self) -> Field {
        match self {
            FieldDef::Simple(t) => Field {
                r#type: t,
                optional: Some(false),
                validate: None,
            },
            FieldDef::Detailed(f) => f,
        }
    }
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Field{
    pub r#type : String,
    #[serde(default)]
    pub optional : Option<bool>,
    #[serde(default)]
    pub validate: Option<HashMap<String, serde_yaml::Value>>,
}