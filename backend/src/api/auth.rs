// src/api/auth.rs

use axum::{
    Json, Router,
    extract::State,
    http::{StatusCode, header::SET_COOKIE},
    response::Response,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

use crate::{
    auth::{Claims, create_token, hash_password, verify_password},
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/status", get(setup_status))
        .route("/setup", post(setup))
        .route("/login", post(login))
        .route("/verify", get(verify))
        .route("/logout", post(logout))
}

// ── Shared response types ─────────────────────────────────────────────────────

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

fn err(msg: impl Into<String>) -> Json<ErrorResponse> {
    Json(ErrorResponse { error: msg.into() })
}

// ── GET /api/auth/status ──────────────────────────────────────────────────────

#[derive(Serialize)]
struct StatusResponse {
    setup_required: bool,
}

async fn setup_status(
    State(state): State<AppState>,
) -> Result<Json<StatusResponse>, (StatusCode, Json<ErrorResponse>)> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM admins")
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                err(format!("DB error: {e}")),
            )
        })?;

    Ok(Json(StatusResponse {
        setup_required: count == 0,
    }))
}

// ── POST /api/auth/setup ──────────────────────────────────────────────────────

#[derive(Deserialize)]
struct SetupRequest {
    username: String,
    password: String,
    email: String,
}

#[derive(Serialize)]
struct SetupResponse {
    message: String,
}

async fn setup(
    State(state): State<AppState>,
    Json(body): Json<SetupRequest>,
) -> Result<(StatusCode, Json<SetupResponse>), (StatusCode, Json<ErrorResponse>)> {
    if body.username.trim().len() < 3 {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            err("Username must be at least 3 characters"),
        ));
    }
    if body.password.len() < 8 {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            err("Password must be at least 8 characters"),
        ));
    }
    if body.email.trim().is_empty() || !body.email.contains('@') {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            err("Valid email is required"),
        ));
    }

    // Block if an admin already exists
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM admins")
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                err(format!("DB error: {e}")),
            )
        })?;

    if count > 0 {
        return Err((StatusCode::CONFLICT, err("An admin account already exists")));
    }

    let hash = hash_password(&body.password);

    sqlx::query("INSERT INTO admins (username, password_hash, email) VALUES (?, ?, ?)")
        .bind(&body.username)
        .bind(&hash)
        .bind(&body.email)
        .execute(&state.db)
        .await
        .map_err(|e| {
            if e.to_string().to_lowercase().contains("duplicate") {
                (StatusCode::CONFLICT, err("Username or email already taken"))
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    err(format!("DB error: {e}")),
                )
            }
        })?;

    Ok((
        StatusCode::CREATED,
        Json(SetupResponse {
            message: "Admin account created. Please sign in.".to_string(),
        }),
    ))
}

// ── POST /api/auth/login ──────────────────────────────────────────────────────

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(sqlx::FromRow)]
struct AdminRow {
    password_hash: String,
}

async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    let row = sqlx::query_as::<_, AdminRow>("SELECT password_hash FROM admins WHERE username = ?")
        .bind(&body.username)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                err(format!("DB error: {e}")),
            )
        })?;

    let valid = match &row {
        Some(r) => verify_password(&body.password, &r.password_hash),
        // Dummy verify keeps response time consistent — prevents user enumeration
        None => {
            let _ = verify_password(
                &body.password,
                "$argon2id$v=19$m=19456,t=2,p=1$dummysaltdummy22$dummyhashvaluedummyhashvaluedummy",
            );
            false
        }
    };

    if !valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            err("Invalid username or password"),
        ));
    }

    sqlx::query("UPDATE admins SET last_login = CURRENT_TIMESTAMP WHERE username = ?")
        .bind(&body.username)
        .execute(&state.db)
        .await
        .ok();

    let token = create_token(&body.username)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, err("Token error")))?;

    let cookie = format!(
        "token={}; Path=/; HttpOnly; SameSite=Strict; {}",
        token,
        if cfg!(debug_assertions) {
            ""
        } else {
            "Secure;"
        }
    );

    let mut res = Response::new("OK".into());
    *res.status_mut() = StatusCode::OK;
    res.headers_mut()
        .insert(SET_COOKIE, cookie.parse().unwrap());

    Ok(res)
}

async fn logout(
    State(_state): State<AppState>,
) -> Response {
    let cookie = "token=; Path=/; HttpOnly; SameSite=Strict; Max-Age=0";

    let mut res = Response::new("OK".into());
    *res.status_mut() = StatusCode::OK;

    res.headers_mut()
        .insert(SET_COOKIE, cookie.parse().unwrap());

    res
}

// ── GET /api/auth/verify ──────────────────────────────────────────────────────

#[derive(Serialize)]
struct VerifyResponse {
    username: String,
}

#[axum::debug_handler]
async fn verify(State(_): State<AppState>, claims: Claims) -> Json<VerifyResponse> {
    Json(VerifyResponse {
        username: claims.sub,
    })
}
