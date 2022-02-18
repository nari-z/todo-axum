use std::sync::Arc;
use std::sync::RwLock;
use std::collections::HashMap;
use uuid::Uuid;
use crate::domain::todo;

pub type DB = Arc<RwLock<HashMap<Uuid, todo::Todo>>>;
