use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Environment error: {0}")]
    Environment(#[from] std::env::VarError),

    #[error("JWT error: {0}")]
    JWT(#[from] jsonwebtokens_cognito::Error),

    #[error("IO error: {0}")]
    IOError(String),
}
