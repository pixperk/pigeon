use std::sync::Arc;

use pigeon_core::registry::HandlerRegistry;
use serde_json::{json, Value};



pub fn register_all() -> Arc<HandlerRegistry>{
     let registry = Arc::new(HandlerRegistry::new());

      // /greet?name=Yash
    registry.register("greet", |input| async move {
        let name = input.get("name").and_then(Value::as_str).unwrap_or("stranger");
        Ok(json!({ "message": format!("Hello, {name}!") }))
    });

    // POST /add { a: 2, b: 3 }
    registry.register("add_numbers", |input| async move {
        let a = input.get("a").and_then(Value::as_i64).unwrap_or(0);
        let b = input.get("b").and_then(Value::as_i64).unwrap_or(0);
        Ok(json!({ "result": a + b }))
    });

    // POST /echo { data: "xyz" }
    registry.register("echo", |input| async move {
        let data = input.get("data").and_then(Value::as_str).unwrap_or("");
        Ok(json!({ "echoed": data }))
    });

    // GET /time
    registry.register("get_time", |_input| async move {
        let now = chrono::Utc::now().to_rfc3339();
        Ok(json!({ "timestamp": now }))
    });


    registry
}