use actix_web::web::{post, put};
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::pool::PoolOptions;
use sqlx::{FromRow, PgPool, Pool, Postgres};
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
#[derive(Serialize, Deserialize)]

struct UpdateTodo {
    completed: bool,
}
async fn add_todo(db: web::Data<PgPool>, json: web::Json<NewTodo>) -> impl Responder {
    println!("📥 Received request to add todo: {:?}", json.title);

    let result = sqlx::query_as::<_, Todo>("INSERT INTO todos (title) VALUES ($1) RETURNING *")
        .bind(&json.title)
        .fetch_one(db.get_ref())
        .await;

    match result {
        Ok(todo) => {
            println!("✅ Todo added with id: {}", todo.id);
            HttpResponse::Created().json(json!({
                "message":"todo added",
                "id": todo.id,
                "todo":todo,
            }))
        }
        Err(e) => {
            println!("❌ Error: {}", e);
            HttpResponse::InternalServerError()
                .body(format!("something wrong running the query: {}", e))
        }
    }
}
async fn delete_todo(data: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query("delete from todos where id=$1")
        .bind(id.into_inner())
        .execute(data.get_ref())
        .await;
    match result {
        Ok(result) => {
            if result.rows_affected() > 0 {
                HttpResponse::Ok().body("todo deleted")
            } else {
                HttpResponse::NotFound().body("todo not found ")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("error updating the db "),
    }
}
async fn update_todo(
    data: web::Data<PgPool>,
    id: web::Path<i32>,
    json: web::Json<UpdateTodo>,
) -> impl Responder {
    let result = sqlx::query("update todos set completed=$1 where id=$2")
        .bind(&json.completed)
        .bind(id.into_inner())
        .execute(data.get_ref())
        .await;
    match result {
        Ok(result) => {
            if result.rows_affected() > 0 {
                HttpResponse::Ok().body("todo updated")
            } else {
                HttpResponse::NotFound().body("todo not found ")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("error updating the db "),
    }
}
async fn fetch_todos(data: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, Todo>("SELECT * from todos order by id")
        .fetch_all(data.get_ref())
        .await;
    match result {
        Ok(Vec) => HttpResponse::Ok().json(Vec),
        Err(_) => HttpResponse::InternalServerError().body("error updating the db "),
    }
}

#[actix_web::main]
pub async fn postgresql_todo() -> io::Result<()> {
    //let db_url = std::env::var("DATABASE_URL").expect("❌ DATABASE_URL not set");
    let url = "postgres://postgres:password@127.0.0.1:5433/todos";
    let db = PgPool::connect(&url)
        .await
        .expect("failed to connect to DB");
    // let db: Pool<Postgres> = PoolOptions::new()
    //     .max_connections(5)
    //     .connect_lazy(&url)
    //     .expect("failed to connect to DB");
    println!("connected to DB");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/add", post().to(add_todo))
            .route("/update/{id}", put().to(update_todo))
            .route("/delete/{id}", web::delete().to(delete_todo))
            .route("/todos", web::get().to(fetch_todos))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
