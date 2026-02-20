use std::sync::Arc;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use crate::state::AppState;

fn get_bearer(req: &Request) -> Result<&str, StatusCode> {
    req.headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|token| token.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)
}

async fn get_claims(keyset: &jsonwebtokens_cognito::KeySet, client_id: &str, token: &str) -> Result<serde_json::Value, StatusCode> {
    let verifier = keyset
        .new_id_token_verifier(&[client_id])
        .build()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(keyset.verify(token, &verifier).await.map_err(|_| StatusCode::UNAUTHORIZED)?)
}

pub async fn auth_middleware(State(state): State<Arc<AppState>>, mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let bearer = get_bearer(&req)?;
    let claims = get_claims(&state.keyset, &state.env.client_id, bearer).await?;
    req.extensions_mut().insert(claims);
    let response = next.run(req).await;
    Ok(response)
}
