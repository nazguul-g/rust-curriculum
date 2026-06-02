use actix_web::{App, HttpServer, Responder};
use std::io;


/*async fn greet() -> impl Responder {

}*/
#[actix_web::main]
pub async fn actix_fun() -> io::Result<()> {
    HttpServer::new(|| App::new())
        .bind(("192.168.0.1", 8080))?
        .workers(2)
        .run()
        .await
}
