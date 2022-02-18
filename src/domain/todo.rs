// use serde::{Serialize, Deserialize};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub done: bool,
}
