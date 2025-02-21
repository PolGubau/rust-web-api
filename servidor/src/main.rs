use actix_web::{get, post, web, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("req_body: {}", req_body);
    req_body
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(hello).service(echo))
        .bind(("127.0.0.1", 4001))?
        .run()
        .await
}
