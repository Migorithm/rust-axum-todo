use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::{postgres::PgRow, PgPool, Row};
use uuid::Uuid;

use crate::{
    db::{Store, StoreType},
    models::{self, Todo},
    schema,
};

pub(crate) async fn todos_index<S: Store<Pool = PgPool>>(
    pagination: Option<Query<schema::Pagination>>,
    State(db): State<StoreType<S>>,
) -> impl IntoResponse {
    let store = db.read().await;
    let Query(pagination) = pagination.unwrap_or_default();

    let todos: Vec<Todo> = sqlx::query_as!(
        Todo,
        "SELECT * FROM todo_list LIMIT $1 OFFSET $2",
        pagination.limit.unwrap_or(i64::MAX),
        pagination.offset.unwrap_or(0)
    )
    .fetch_all(store.connection())
    .await
    .unwrap();

    Json(todos)
}

pub(crate) async fn todos_create<S: Store<Pool = PgPool>>(
    State(db): State<StoreType<S>>,
    Json(input): Json<schema::CreateTodo>,
) -> impl IntoResponse {
    let store = db.read().await;

    let todo: Todo = match sqlx::query_as!(
        Todo,
        "INSERT INTO todo_list (text,completed) VALUES ($1, $2) RETURNING *",
        input.text,
        false
    )
    .fetch_one(store.connection())
    .await
    {
        Ok(val) => val,
        Err(err) => {
            dbg!(err);
            panic!("Whathappen?")
        }
    };

    (StatusCode::CREATED, Json(todo))
}

pub(crate) async fn todos_update<S: Store<Pool = PgPool>>(
    Path(id): Path<Uuid>,
    State(db): State<StoreType<S>>,
    Json(input): Json<schema::UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let store = db.read().await;

    let todo: Todo = match sqlx::query_as!(
        Todo,
        "UPDATE todo_list SET text = $1 WHERE id = $2 RETURNING *",
        input.text.unwrap(),
        id
    )
    .fetch_one(store.connection())
    .await
    {
        Ok(val) => val,
        Err(err) => {
            dbg!(err);
            panic!("What happened?!")
        }
    };

    Ok(Json(todo))
}

// pub(crate) async fn todos_delete<S: Store<Pool = PgPool>>(
//     Path(id): Path<Uuid>,
//     State(db): State<StoreType<S>>,
// ) -> impl IntoResponse {
//     if db.write().unwrap().remove(&id).is_some() {
//         StatusCode::NO_CONTENT
//     } else {
//         StatusCode::NOT_FOUND
//     }
// }
