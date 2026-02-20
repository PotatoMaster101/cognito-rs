use std::net::{IpAddr, SocketAddr};
use crate::error::AppError;

/// Represents the application environment.
#[derive(Clone, Debug)]
pub struct AppEnvironment {
    pub backend_addr: String,
    pub backend_log_filter: String,
    pub backend_port: String,
    pub cert_dir: String,
    pub client_id: String,
    pub pool_id: String,
    pub region: String,
}

impl AppEnvironment {
    /// Loads the application environment from environment variables.
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(AppEnvironment {
            backend_addr: std::env::var("BACKEND_ADDR")?,
            backend_log_filter: std::env::var("BACKEND_LOG_FILTER")?,
            backend_port: std::env::var("VITE_API_PORT")?,
            cert_dir: std::env::var("VITE_CERT_DIR")?,
            client_id: std::env::var("VITE_COGNITO_CLIENT_ID")?,
            pool_id: std::env::var("COGNITO_POOL_ID")?,
            region: std::env::var("COGNITO_REGION")?,
        })
    }

    /// Gets the backend address and port.
    pub fn get_addr(&self, default_addr: IpAddr, default_port: u16) -> SocketAddr {
        let addr = self.backend_addr.parse().unwrap_or(default_addr);
        let port = self.backend_port.parse().unwrap_or(default_port);
        SocketAddr::new(addr, port)
    }
}

/// Represents the application state.
#[derive(Clone, Debug)]
pub struct AppState {
    pub env: AppEnvironment,
    pub keyset: jsonwebtokens_cognito::KeySet,
}

impl AppState {
    /// Creates a new application state.
    pub fn new() -> Result<Self, AppError> {
        let env = AppEnvironment::from_env()?;
        let keyset = jsonwebtokens_cognito::KeySet::new(&env.region, &env.pool_id)?;
        Ok(Self { env, keyset })
    }
}
