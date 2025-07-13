use std::sync::Arc;

use pigeon_core::registry::HandlerRegistry;
use serde_json::{json, Value};



pub fn register_all() -> Arc<HandlerRegistry>{
     let registry = Arc::new(HandlerRegistry::new());

     registry.register("greet", |input| async move {
        let name = input.get("name").and_then(Value::as_str).unwrap_or("stranger");
        Ok(json!({ "message": format!("Hello, {name}") }))
    });

    registry
}