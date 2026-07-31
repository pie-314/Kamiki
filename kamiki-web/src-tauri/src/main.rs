mod api;

use axum::{
    routing::{get, post},
    Router,
};
use clap::Parser;
use std::net::SocketAddr;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    interface: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let args = Args::parse();
    
    if let Some(iface) = args.interface {
        unsafe {
            std::env::set_var("KAMIKI_INTERFACE", iface);
        }
    }

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/interfaces", get(api::get_interfaces))
        .route("/api/start", post(api::start_capture))
        .route("/api/stop", post(api::stop_capture))
        .route("/api/poll", get(api::poll_packets))
        .route("/api/flows", get(api::get_flows))
        .route("/api/stats", get(api::get_stats))
        .fallback_service(ServeDir::new("target/dx/kamiki-web-ui/debug/web/public"))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    log::info!("Server listening on http://{}", addr);

    // Auto-open browser
    let url = format!("http://{}", addr);
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        if let Err(e) = open::that(&url) {
            log::error!("Failed to open browser: {}", e);
        }
    });

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
