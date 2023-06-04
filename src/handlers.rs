use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    db::{Store, StoreType},
    models::Todo,
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

    let mut todo = sqlx::query_as!(Todo, "SELECT * FROM todo_list WHERE id = $1", id)
        .fetch_one(store.connection())
        .await
        .unwrap();

    if let Some(t) = input.text {
        todo.text = t;
    }
    if let Some(c) = input.completed {
        todo.completed = c;
    }
    println!("UPDATE QUERY ON ID: {}", id);
    let todo = sqlx::query_as!(
        Todo,
        "UPDATE todo_list SET text = $1, completed = $2 WHERE id = $3 RETURNING *",
        todo.text,
        todo.completed,
        todo.id
    )
    .fetch_one(store.connection())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::ACCEPTED, Json(todo)))
}

pub(crate) async fn todos_delete<S: Store<Pool = PgPool>>(
    Path(id): Path<Uuid>,
    State(db): State<StoreType<S>>,
) -> impl IntoResponse {
    let store = db.read().await;
    sqlx::query_as!(Todo, "DELETE FROM todo_list where id =$1", id)
        .execute(store.connection())
        .await
        .unwrap();

    StatusCode::ACCEPTED
}
