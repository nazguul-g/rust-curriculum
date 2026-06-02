use actix_web::{App, HttpResponse, HttpServer, Responder, Result, error, get, post, web};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::io;
use std::sync::Mutex;

#[derive(Deserialize, Serialize)]
struct GreetingRequest {
    name: String,
}
#[derive(Deserialize, Serialize)]
struct GreetingResponse {
    name: String,
    visitor: u64,
}
#[derive(Serialize)]
struct HealthCheckResponse {
    status: String,
    version: String,
}

struct AppState {
    visitors_counter: Mutex<u64>,
}
#[derive(Debug)]
struct AppErr {
    msg: String,
}
impl error::ResponseError for AppErr {}
impl Display for AppErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[get("/")]
async fn hello(data: web::Data<AppState>) -> Result<impl Responder> {
    let mut counter = data.visitors_counter.lock().unwrap();
    *counter += 1;
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            r#"<h1>👋 Hello, Rust Web!</h1>
            <p>Try these endpoints:</p>
            <ul>
                <li><a href="/health">/health</a> - Health check</li>
                <li><a href="/greet">/greet</a> - Greeting form</li>
                <li><a href="/api/greet?name=Visitor">/api/greet?name=Visitor</a> - JSON API</li>
                <li><a href="/metrics">/metrics</a> - Visitor count</li>
            </ul>"#,
        ))
}
#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthCheckResponse {
        status: "Ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

#[get("/greet")]
async fn greet_form() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            r#"
        <form action="/api/greet" method="get">
                <label for="name">Your name:</label>
                <input type="text" id="name" name="name" required>
                <button type="submit">Greet me!</button>
            </form>
        "#,
        )
}

#[get("/api/greet")]
async fn greet_api(
    data: web::Data<AppState>,
    query: web::Query<GreetingRequest>,
) -> Result<impl Responder> {
    let mut counter = data.visitors_counter.lock().unwrap();
    *counter += 1;
    Ok(HttpResponse::Ok().json(GreetingResponse {
        name: format!("Hello {}", query.name),
        visitor: *counter,
    }))
}
async fn not_found() -> Result<HttpResponse, AppErr> {
    Err(AppErr {
        msg: r#"Resource not found"#.to_string(),
    })
}

#[actix_web::main]
pub async fn hello_actix() -> io::Result<()> {
    println!("starting server at http://127.0.0.1:8080");
    let app_data = web::Data::new(AppState {
        visitors_counter: Mutex::new(0),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(hello)
            .service(health_check)
            .service(greet_form)
            .service(greet_api)
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
