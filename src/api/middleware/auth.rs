use axum::{
    Json,
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next, response::IntoResponse,
};

// use crate::{api_response::ApiResponse, app_state::AppStateStore, claims::Claims};
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::{api::{api_response::ApiResponse, claims::Claims}, app_state::AppStateStore};

pub async fn auth_middleware(
    State(state): State<AppStateStore>,
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse>)> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok());

    let Some(auth) = auth_header else {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::respond(StatusCode::UNAUTHORIZED)),
        ));
    };

    let token = auth.strip_prefix("Bearer ").unwrap_or("");

    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.read().await.secret.as_bytes()),
        &Validation::default(),
    );

    let _ = decoded.map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::respond(StatusCode::UNAUTHORIZED)),
        )
    })?;

    Ok(next.run(req).await)
}
