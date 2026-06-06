use actix_web::web::{get, post};
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::sqlite::SqlitePool;
use std::io;
#[derive(Serialize, FromRow)]
struct Post {
    id: i64,
    title: String,
    content: String,
}
#[derive(Deserialize)]
struct NewPost {
    title: String,
    content: String,
}

async fn get_posts(db: web::Data<SqlitePool>) -> impl Responder {
    let posts = sqlx::query_as::<_, Post>("SELECT * FROM posts")
        .fetch_all(db.get_ref())
        .await;

    match posts {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("error geting posts front the db,{e}"))
        }
    }
}
async fn add_post(post: web::Json<NewPost>, db: web::Data<SqlitePool>) -> impl Responder {
    let post = post.into_inner();
    let result = sqlx::query("INSERT INTO posts (title, content) values (?,?)")
        .bind(post.title)
        .bind(post.content)
        .execute(db.as_ref())
        .await;

    match result {
        Ok(resul) => HttpResponse::Created().body("posts added into db successfully"),
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("error getting posts front the db,{e}")),
    }
}

#[actix_web::main]
pub async fn sql_actix() -> io::Result<()> {
    let pool = SqlitePool::connect("assets/sqlite.db")
        .await
        .expect("error connecting to DB");

    let result = sqlx::query(
        "CREATE TABLE IF NOT EXISTS posts (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        title TEXT NOT NULL,
        content TEXT NOT NULL
    )",
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");
    let _ = sqlx::query("INSERT INTO posts (title,content)
                             values (\"the nazguul take the full control ready up\",\"holly fuck nazguul is goated\")")
        .execute(&pool).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/posts", get().to(get_posts))
            .route("/api/add_post", post().to(add_post))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
