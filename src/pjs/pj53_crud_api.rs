use actix_web::{App, HttpResponse, HttpServer, Responder, delete, get, post, put, web};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;
use std::sync::Mutex;

#[derive(Clone, Serialize, Deserialize)]
struct BlogPost {
    content: String,
}
struct AppState {
    blogs: Mutex<HashMap<u64, BlogPost>>,
    next_id: Mutex<u64>,
}

#[get("/blogs")]
async fn get_blogs(data: web::Data<AppState>) -> impl Responder {
    let blogs = data.blogs.lock().unwrap();
    let values: Vec<_> = blogs.values().cloned().collect();
    HttpResponse::Ok().json(values)
}
#[get("/blogs/{id}")]
async fn get_post(data: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let blogs = data.blogs.lock().unwrap();
    let id = id.into_inner();
    match blogs.get(&id) {
        Some(value) => HttpResponse::Ok().json(value),
        None => HttpResponse::NotFound().body("post not found"),
    }
}

#[post("/blog")]
async fn post_blogpost(data: web::Data<AppState>, blog: web::Json<BlogPost>) -> impl Responder {
    let mut blogs = data.blogs.lock().unwrap();
    let id = {
        let mut next_id = data.next_id.lock().unwrap();
        let id = *next_id;
        *next_id += 1;
        id
    };
    blogs.insert(id, blog.into_inner());
    HttpResponse::Created().body("blog inserted")
}
#[put("/blog/{id}")]
async fn update_post(
    data: web::Data<AppState>,
    blog: web::Json<BlogPost>,
    id: web::Path<u64>,
) -> impl Responder {
    let mut blogs = data.blogs.lock().unwrap();
    let id = id.into_inner();
    if let Some(existing_blog) = blogs.get_mut(&id) {
        *existing_blog = blog.into_inner();
        HttpResponse::Ok().body("blog updated")
    } else {
        HttpResponse::NotFound().body("blog not found")
    }
}
#[delete("/blog/{id}")]
async fn delete_post(data: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let mut blogs = data.blogs.lock().unwrap();
    let id = id.into_inner();
    if let Some(value) = blogs.remove(&id) {
        HttpResponse::Ok().body("blog post removed")
    } else {
        HttpResponse::NotFound().body("blog post not found")
    }
}
#[actix_web::main]
pub async fn crud_api() -> io::Result<()> {
    let blogs = web::Data::new(AppState {
        blogs: Mutex::new(HashMap::new()),
        next_id: Mutex::new(0),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(blogs.clone())
            .service(get_blogs)
            .service(get_post)
            .service(delete_post)
            .service(update_post)
            .service(post_blogpost)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
