use actix_web::web::post;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{FromRow, PgPool};
use std::io;

#[derive(FromRow, Serialize, Deserialize)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

#[derive(Serialize, Deserialize)]
struct NewTodo {
    title: String,
}
struct UpdateTodo {
    completed: bool,
}
async fn add_todo(db: web::Data<PgPool>, json: web::Json<NewTodo>) -> impl Responder {
    let result = sqlx::query_as::<_, Todo>("INSERT INTO todos (title) VALUES ($1) RETURNING *")
        .bind(&json.title)
        .fetch_one(db.get_ref())
        .await;
    match result {
        Ok(todo) => HttpResponse::Created().json(json!({
            "message":"todo added",
            "id": todo.id,
            "todo":todo,
        })),
        Err(_) => HttpResponse::InternalServerError().body("something wrong running the query"),
    }
}
async fn delete_todo(id: web::Path<u64>) -> impl Responder {
    HttpResponse::Ok()
}
async fn update_todo(
    data: web::Data<PgPool>,
    id: web::Path<u64>,
    json: web::Json<UpdateTodo>,
) -> impl Responder {
    HttpResponse::Ok()
}
async fn fetch_todos(data: web::Data<PgPool>) -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
pub async fn postgresql_todo() -> io::Result<()> {
    let db_url = std::env::var("DATABASE_URL").expect("❌ DATABASE_URL not set");

    let pool = PgPool::connect(&db_url)
        .await
        .expect("failed to connect to DB");
    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .route("/add_todo", post().to(add_todo))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
