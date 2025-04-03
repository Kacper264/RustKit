mod app;
mod config;
pub mod middleware;
pub mod handlers;

#[tokio::main]
async fn main() {
    // Initialisation du logger
    env_logger::init();

    // Chargement de la configuration
    let addr = config::get_server_address();

    // Lancer l'application
    app::run(addr).await;
}