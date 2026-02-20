mod auth;
mod error;
mod handler;
mod state;

use std::sync::Arc;
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use tower_http::services::{ServeDir, ServeFile};
use crate::error::AppError;
use crate::state::AppState;

fn get_router(state: Arc<AppState>) -> Router {
    let api = Router::new()
        .route("/", axum::routing::get(handler::handle_authenticated))
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth::auth_middleware))
        .fallback(|| async { axum::http::StatusCode::NOT_FOUND });

    let serve_dir = ServeDir::new("dist")
        .fallback(ServeFile::new("dist/index.html"));

    let middleware = tower::ServiceBuilder::new()
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(tower_http::compression::CompressionLayer::new())
        .layer(tower_http::cors::CorsLayer::permissive());

    Router::new()
        .nest("/api", api)
        .fallback_service(serve_dir)
        .layer(middleware)
        .with_state(state.clone())
}

async fn get_tls_config(cert_dir: impl AsRef<std::path::Path>) -> Result<RustlsConfig, AppError> {
    let cert_path = cert_dir.as_ref().join("cert.pem");
    let key_path = cert_dir.as_ref().join("key.pem");
    RustlsConfig::from_pem_file(cert_path, key_path)
        .await
        .map_err(|_| AppError::IOError("Failed to load TLS cert".to_string()))
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let state = Arc::new(AppState::new().expect("Failed to create app state"));
    let addr = state.env.get_addr(std::net::IpAddr::from([0, 0, 0, 0]), 9999);
    let cert_dir = shellexpand::tilde(&state.env.cert_dir);
    let tls = get_tls_config(cert_dir.as_ref()).await.expect("Failed to load TLS");
    let router = get_router(state.clone());

    tracing_subscriber::fmt().with_env_filter(&state.env.backend_log_filter).init();
    tracing::info!("Listening on {}", addr);
    axum_server::bind_rustls(addr, tls)
        .serve(router.into_make_service())
        .await
        .expect("Failed to start server");
}
