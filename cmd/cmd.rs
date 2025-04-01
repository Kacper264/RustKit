use hyper::{service::{make_service_fn, service_fn}, Body, Request, Response, Server, Error};
use prometheus::{Encoder, TextEncoder, Registry, IntCounter};
use std::{sync::Arc, net::SocketAddr};
use tokio::{signal, sync::oneshot};
use log::{info, error};

#[tokio::main]
async fn main() {
    // Initialisation du logger
    env_logger::init();

    // Création de l'application et de ses métriques
    let (counter, registry) = create_metrics();

    // Utilisation d'Arc pour s'assurer que counter a une durée de vie suffisante
    let counter = Arc::new(counter);

    // Création du serveur HTTP
    let make_svc = make_service_fn(move |_conn| {  // Utilisation de `move` pour déplacer `counter` dans la closure
        let counter = Arc::clone(&counter); // Clone l'Arc pour partager entre les tâches
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                handle_request(req, Arc::clone(&counter))  // Partage du compteur entre les requêtes
            }))
        }
    });

    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let server = Server::bind(&addr)
        .serve(make_svc);

    // Créer un canal pour gérer l'arrêt du serveur
    let (tx, rx) = oneshot::channel::<()>();

    // Tâche pour gérer l'arrêt sur signal Ctrl+C
    let signal_task = tokio::spawn(async move {
        signal::ctrl_c().await.unwrap();
        info!("Received shutdown signal");
        let _ = tx.send(()); // Envoie le signal d'arrêt
    });

    // Lancer le serveur HTTP dans une tâche asynchrone
    let server_task = tokio::spawn(async {
        if let Err(e) = server.await {
            error!("Server error: {}", e);
        }
    });

    // Attendre la réception du signal d'arrêt
    rx.await.unwrap();

    // Une fois le signal reçu, arrête le serveur
    server_task.abort();
    signal_task.await.unwrap();

    info!("Server stopped");
}

fn create_metrics() -> (IntCounter, Registry) {
    // Création d'un compteur de requêtes avec un AtomicU64
    let counter = IntCounter::new(
        "user_api_request_count",
        "Number of requests received"
    ).unwrap();

    let registry = Registry::new();
    // Enregistrement du compteur dans le registre Prometheus
    registry.register(Box::new(counter.clone())).unwrap();

    (counter, registry)
}

async fn handle_request(req: Request<Body>, counter: Arc<IntCounter>) -> Result<Response<Body>, Error> {
    // Incrémentation du compteur
    counter.inc();

    // Si c'est une requête pour "/metrics", on retourne "Hello, World!"
    if req.uri().path() == "/hello" {
        return Ok(Response::new(Body::from("Hello, World!\n")));
    }

    // Exemple de réponse pour toutes les autres routes
    let response = Response::new(Body::from("Hello, World!"));

    Ok(response)
}
