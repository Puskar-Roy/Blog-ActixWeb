use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder};
mod utlils;
mod routes;


#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] 
async fn main() -> std::io::Result<()> {

    if std::env::var_os("RUST_LOG").is_none(){
        std::env::set_var("RUST_LOG", "actix_web=info");

    }
    dotenv::dotenv().ok();
    env_logger::init();

    let port = (*utlils::constants::PORT).clone();
    let address = (*utlils::constants::ADDRESS).clone();


    HttpServer::new(|| App::new()
    .wrap(Logger::default())
    .service(greet))
        .bind((address, port))?
        .run()
        .await
}
