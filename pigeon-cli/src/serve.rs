use std::{fs, net::SocketAddr, sync::Arc};

use axum::{routing::{delete, get, patch, post, put}, Json, Router};
use pigeon_core::{registry::{runtime::handle_rpc, HandlerRegistry}, schema::Schema};
use serde_json::{Value};



pub async fn run_server(file: &str, registry : Arc<HandlerRegistry>) -> anyhow::Result<()> {
    let content = fs::read_to_string(file)?;
    let schema: Schema = serde_yaml::from_str(&content)?;
    let schema = Arc::new(schema);

    let mut app: Router = Router::new();

    
    for rpc in &schema.rpcs {
        let path = rpc.path.clone();
        let rpc_name = rpc.name.clone();

        let schema = Arc::clone(&schema);
        let registry = Arc::clone(&registry);

        //Axum handler
        let handler = move |Json(body): Json<Value>| {
           
            let schema = Arc::clone(&schema);
            let registry = Arc::clone(&registry);
            let rpc_name = rpc_name.clone();
            async move {
               handle_rpc(rpc_name, body, schema, registry).await
            }
        };

        app = match rpc.method.as_str() {
            "POST" => {
                app.route(&path, post(handler))
            },
            "GET" => {
                app.route(&path, get(handler))
            },
            "DELETE" =>{
                app.route(&path, delete(handler))
            },
            "PATCH" => {
                app.route(&path, patch(handler))
            },
            "PUT" => {
                app.route(&path, put(handler))
            },
            _ => app,
        };
    }

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("🕊️  Pigeon flying at http://{}", addr);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;

    Ok(())
}
