use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub(crate) struct Todo {
    pub(crate) id: Uuid,
    pub(crate) text: String,
    pub(crate) completed: bool,
}

#[derive(Debug, Serialize)]
pub(crate) struct TodoId {
    pub(crate) id: Uuid,
}
