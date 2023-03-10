use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::db::{open_db_connection, user::add_user};

#[derive(Deserialize, Serialize)]
struct AddUserDTO {
    email: String,
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
