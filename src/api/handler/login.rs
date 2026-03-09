use axum::{Json, extract::State, http::StatusCode};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Deserialize;
use serde_json::json;

use crate::{api::api_response::{ApiResponse, ResultApiResponse}, app_state::AppStateStore, api::claims::Claims, app_config::config};

#[derive(Deserialize)]
pub struct LoginInput {
    username: String,
    password: String,
}

pub async fn login_handler(
    State(state): State<AppStateStore>,
    Json(body): Json<LoginInput>,
) -> ResultApiResponse {
    let username = &config().username;
    let password = &config().password;
    if body.username != *username || body.password != *password {
        return Err((StatusCode::UNAUTHORIZED, Json(ApiResponse::error("Wrong username or password.", StatusCode::UNAUTHORIZED.as_u16()))));
    }

    let expiration = (Utc::now() + Duration::hours(24)).timestamp() as usize;

    let claims = Claims {
        sub: body.username,
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.read().await.secret.as_bytes()),
    )
    .unwrap();

    Ok(Json(ApiResponse::ok_data(
        "Login successfull.",
        json!({ "token": token }),
    )))
}