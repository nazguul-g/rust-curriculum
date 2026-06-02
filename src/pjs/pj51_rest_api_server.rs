use actix_web::{App, HttpResponse, HttpServer, Responder, Result, get, post, web};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io;
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct AppState {
    books: Mutex<Vec<String>>,
}
#[derive(Serialize, Deserialize)]
struct BookRequest {
    book_name: String,
}
#[get("/books")]
async fn get_books(data: web::Data<AppState>) -> Result<impl Responder> {
    let books_vec = data.books.lock().unwrap();
    let html = format!(
        "{}{}",
        json!(&*books_vec),
        r#"
        <form action="/api/books" method="post">
            <label for="name">add book:</label>
            <input type="text" id="name" name="name" required>
            <button type="submit">Add</button>
        </form>
        "#
    );
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset: utf-8")
        .body(html))
}
#[post("/api/books")]
async fn books_api(
    query: web::Query<BookRequest>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    let mut books_vec = data.books.lock().unwrap();
    books_vec.push(query.book_name.clone());
    Ok(HttpResponse::Ok())
}

#[actix_web::main]
pub async fn rest_api_server() -> io::Result<()> {
    let books_vector = web::Data::new(AppState {
        books: Mutex::new(vec!["book1".to_string(), "book2".to_string()]),
    });
    println!("server started at http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(books_vector.clone())
            .service(get_books)
            .service(books_api)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
