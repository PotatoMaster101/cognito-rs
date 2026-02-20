use axum::Extension;
use crate::auth::AuthUser;

pub async fn handle_authenticated(Extension(user): Extension<AuthUser>) -> impl axum::response::IntoResponse {
    format!("email: {}\nsub: {}\nverified: {}", user.email, user.sub, user.email_verified)
}
