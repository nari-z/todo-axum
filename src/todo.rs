use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
    extract::Extension,
};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::domain::todo;
use crate::db;

#[derive(Debug, Deserialize, Clone)]
pub struct CreateTodoRequest {
    title: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct CreateTodoResponse {
    id: Uuid,
    title: String,
    done: bool,
}

pub async fn create(
    Json(params): Json<CreateTodoRequest>,
    Extension(db): Extension<db::DB>,
) -> impl IntoResponse {
    let todo = todo::Todo {
        id: Uuid::new_v4(),
        title: params.title,
        done: false,
    };

    db.write().unwrap().insert(todo.id, todo.clone());

    (
        StatusCode::CREATED,
        Json(
            CreateTodoResponse{
                id: todo.id,
                title: todo.title,
                done: todo.done,
            },
        ),
    )
}
