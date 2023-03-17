use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    SignedCookieJar,
};
use serde::Deserialize;

use crate::db::{
    open_db_connection,
    user::{add_user, validate_user, UserID},
};

const SESSION_COOKIE_KEY: &str = "uid";

#[derive(Deserialize)]
struct AddUserDTO {
    email: String,
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginDTO {
    username: String,
    password: String,
}

pub async fn create_new_user(Json(payload): Json<serde_json::Value>) -> impl IntoResponse {
    let data: AddUserDTO = match serde_json::from_value(payload) {
        Ok(data) => data,
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                "Error parsing provided JSON object",
            ))
        }
    };
    let conn = &mut open_db_connection();
    let id = match add_user(conn, &data.username, &data.email, &data.password) {
        Ok(id) => id,
        // TODO: downcast anyhow error, check for rustqlite constraint violation,
        // and print seperate error message on that
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error adding new user to database",
            ))
        }
    };

    Ok((StatusCode::CREATED, id.0.to_string()))
}

pub async fn login_user(
    jar: SignedCookieJar,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let data: LoginDTO = match serde_json::from_value(payload) {
        Ok(data) => data,
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                "Error parsing provided JSON object",
            ))
        }
    };
    let conn = &mut open_db_connection();
    let uid = match validate_user(conn, &data.username, &data.password) {
        Ok(uid) => uid,
        Err(_) => return Err((StatusCode::UNAUTHORIZED, "Invalid username or password")),
    };
    let cookie = Cookie::build(SESSION_COOKIE_KEY, uid.0.to_string())
        .http_only(true)
        .same_site(SameSite::Strict)
        .finish();
    Ok((StatusCode::OK, jar.add(cookie)))
}

pub async fn get_login(jar: SignedCookieJar) -> impl IntoResponse {
    let Some(_) = read_user_cookie(jar) else {
         return Err((StatusCode::UNAUTHORIZED, "You are not currently logged in"));
    };
    Ok(StatusCode::NO_CONTENT)
}

pub async fn logout_user(jar: SignedCookieJar) -> impl IntoResponse {
    let cookie = match jar.get(SESSION_COOKIE_KEY) {
        Some(c) => c,
        None => return Err((StatusCode::UNAUTHORIZED, "You are not currently logged in")),
    };
    Ok((StatusCode::NO_CONTENT, jar.remove(cookie)))
}

pub fn read_user_cookie(jar: SignedCookieJar) -> Option<UserID> {
    let cookie = jar.get(SESSION_COOKIE_KEY)?;
    Some(UserID(cookie.value().parse().ok()?))
}
