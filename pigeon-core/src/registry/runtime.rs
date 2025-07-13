use std::sync::Arc;

use axum::{Json, response::IntoResponse};

use serde_json::{Value, json};

use crate::{registry::HandlerRegistry, schema::Schema};

pub async fn handle_rpc(
    rpc_name: String,
    body: Value,
    schema: Arc<Schema>,
    registry: Arc<HandlerRegistry>,
) -> axum::response::Response {
    let rpc = schema.rpcs.iter().find(|r| r.name == rpc_name);

    if rpc.is_none() {
        return Json(json!({ "error": "Unknown RPC" })).into_response();
    }

    match registry.get(&rpc_name) {
        Some(handler) => match handler(body).await {
            Ok(resp) => Json(resp).into_response(),
            Err(e) => Json(json!({ "error": e })).into_response(),
        },
        None => Json(json!({ "error": "Handler not implemented" })).into_response(),
    }
}
