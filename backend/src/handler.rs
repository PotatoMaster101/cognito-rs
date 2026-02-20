use axum::Extension;

pub async fn handle_authenticated(Extension(claims): Extension<serde_json::Value>) -> impl axum::response::IntoResponse {
    let user = claims.get("email")
        .and_then(|email| email.as_str())
        .unwrap_or("unknown");

    format!("authenticated user: {}", user)
}
