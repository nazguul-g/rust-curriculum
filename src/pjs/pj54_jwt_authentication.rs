use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use bcrypt::verify;
use jsonwebtoken;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io;
use std::sync::Mutex;

const SECRET_KEY: &[u8] = b"aranndomsecretkey";
struct AppState {
    users: Mutex<Vec<User>>,
}

struct User {
    username: String,
    hashed_password: String,
}
#[derive(Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}
#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: u64,
}

async fn login(data: web::Data<AppState>, req: web::Json<LoginRequest>) -> impl Responder {
    let users = data.users.lock().unwrap();
    let credentials = req.into_inner();
    let user_found = users
        .iter()
        .find(|user| user.username == credentials.username);
    return if let Some(user) = user_found {
        let is_ok = verify(&credentials.password, &user.hashed_password).unwrap();
        if !is_ok {
            return HttpResponse::Unauthorized().body("password is wrong");
        }
        let claims = Claims {
            sub: user.username.clone(),
            exp: chrono::Utc::now().timestamp() as u64 + 3600,
        };
        let token = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(SECRET_KEY),
        )
        .unwrap();
        HttpResponse::Ok().json(json!(token))
    } else {
        HttpResponse::NotFound().body("user not found")
    };
}
async fn protected(req: HttpRequest) -> impl Responder {
    let req_token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());
    if let Some(token_value) = req_token {
        if let Some(token) = token_value.strip_prefix("Bearer ") {
            let validation = Validation::new(Algorithm::HS256);
            let data = jsonwebtoken::decode::<Claims>(
                &token,
                &DecodingKey::from_secret(SECRET_KEY),
                &validation,
            );
            return match data {
                Ok(token) => HttpResponse::Ok().body("authorized"),
                Err(_) => HttpResponse::Unauthorized().body("unauthorized"),
            };
        }
    }
    HttpResponse::BadRequest().body("token not found")
}

#[actix_web::main]
pub async fn jwt() -> io::Result<()> {
    // didnt hash the pwd for educationel persposes, and reduce overhead
    let users = web::Data::new(AppState {
        users: Mutex::new(vec![User {
            username: "nazguul".to_string(),
            hashed_password: bcrypt::hash("123", bcrypt::DEFAULT_COST).unwrap(),
        }]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(users.clone())
            .route("/login", web::post().to(login))
            .route("/protected", web::post().to(protected))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
