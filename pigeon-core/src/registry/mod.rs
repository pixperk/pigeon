pub mod runtime;

use std::{collections::HashMap, sync::{Arc, RwLock}};

use serde_json::Value;

 pub type HandlerFn = Arc<dyn Fn(Value) -> HandlerFuture + Send + Sync>;
 pub type HandlerFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Value, String>> + Send>>;

 #[derive(Default, Clone)]
pub struct HandlerRegistry {
    handlers: Arc<RwLock<HashMap<String, HandlerFn>>>,
}

impl HandlerRegistry{
    pub fn new() -> Self{
        Self { 
        handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register<F, Fut>(&self, name: &str, f : F)
    where 
    F : Fn(Value) -> Fut + Send + Sync + 'static,
    Fut : std::future::Future<Output = Result<Value, String>> + Send + 'static,
    {
        let mut map = self.handlers.write().unwrap();
        map.insert(name.to_string(), Arc::new(move |v| Box::pin(f(v))));
    }

    pub fn get(&self, name : &str) -> Option<HandlerFn>{
        self.handlers.read().unwrap().get(name).cloned()
    }
}