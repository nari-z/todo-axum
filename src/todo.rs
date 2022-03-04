use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
    extract::{Extension,Path},
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

#[derive(Debug, Serialize, Clone)]
pub struct FindTodoResponse {
    id: Uuid,
    title: String,
    done: bool,
}

pub async fn find(
    Path(id): Path<Uuid>,
    Extension(db): Extension<db::DB>,
) -> impl IntoResponse {
    match db.read().unwrap().get(&id) {
        Some(result) => (
            StatusCode::OK,
            Json(
                FindTodoResponse{
                    id: result.id,
                    title: result.title.clone(),
                    done: result.done,
                },
            ),
        ),
        // TODO: 空の struct を返却したくない。message だけ返却できればよい
        None => (
            StatusCode::NOT_FOUND,
            Json(
                FindTodoResponse{
                    id: Uuid::default(),
                    title: "".to_string(),
                    done: false,
                },
            ),
        ),
    }
}

