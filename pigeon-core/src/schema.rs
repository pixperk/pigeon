use serde::Deserialize;
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
    pub request : HashMap<String, String>,
    pub response : HashMap<String, String>
}