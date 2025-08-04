use actix_web::{get, middleware::Logger, post, web::Json, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidJson {
    pub planet: String
}


#[post("/")]
async fn post_hello(posted: Json<ValidJson>) -> impl Responder {
    let planet = posted.planet.clone();
    HttpResponse::Ok().body(format!("Hello, {}!", planet))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    tracing_subscriber::fmt().init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .service(post_hello)
    }).bind(("0.0.0.0", 5120))?
    .run().await
}
