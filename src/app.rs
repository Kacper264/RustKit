use crate::handlers;  
use crate::middleware;
use hyper::{service::{make_service_fn, service_fn}, Server};
use log::{error, info};
use std::{net::SocketAddr, sync::Arc};
use tokio::{signal, sync::oneshot};

use handlers::user::handle_request;
use middleware::metrics::create_metrics;

pub async fn run(addr: SocketAddr) {
    let (counter, _) = create_metrics();
    let counter = Arc::new(counter);

    let make_svc = make_service_fn(move |_conn| {
        let counter = Arc::clone(&counter);
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                handle_request(req, Arc::clone(&counter))
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);
    let (tx, rx) = oneshot::channel::<()>();

    let signal_task = tokio::spawn(async move {
        signal::ctrl_c().await.unwrap();
        info!("Received shutdown signal");
        let _ = tx.send(());
    });

    let server_task = tokio::spawn(async {
        if let Err(e) = server.await {
            error!("Server error: {}", e);
        }
    });

    rx.await.unwrap();
    server_task.abort();
    signal_task.await.unwrap();

    info!("Server stopped");
}

