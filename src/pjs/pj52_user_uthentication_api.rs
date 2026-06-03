use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use bcrypt::{DEFAULT_COST, verify};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct SignUpRequest {
    username: String,
    password: String,
}
#[derive(Serialize, Deserialize)]

struct LoginRequest {
    username: String,
    password: String,
}
struct User {
    username: String,
    hashed_password: String,
}
struct AppState {
    users: Mutex<HashMap<String, User>>,
}
async fn signup(data: web::Data<AppState>, request: web::Json<SignUpRequest>) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    if users.contains_key(&request.username) {
        return HttpResponse::Conflict().body("user name already exists");
    }
    let hashed_password = bcrypt::hash(&request.password, DEFAULT_COST).unwrap();
    let user = User {
        username: request.username.clone(),
        hashed_password,
    };
    users.insert(request.username.clone(), user);
    HttpResponse::Created().body("user created")
}
async fn login(data: web::Data<AppState>, request: web::Json<LoginRequest>) -> impl Responder {
    let users = data.users.lock().unwrap();
    match users.get(&request.username) {
        Some(user) => {
            if verify(&request.password, &user.hashed_password).unwrap_or(false) {
                HttpResponse::Ok().body("user found")
            } else {
                HttpResponse::Unauthorized().body("password wrong")
            }
        }
        None => HttpResponse::NotFound().body("user not found"),
    }
}
#[actix_web::main]
pub async fn auth_api() -> io::Result<()> {
    let users = web::Data::new(AppState {
        users: Mutex::new(HashMap::new()),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(users.clone())
            .route("/login", web::post().to(login))
            .route("/signup", web::post().to(signup))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
