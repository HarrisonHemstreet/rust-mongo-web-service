mod routes;
use crate::routes::{echo, hello, manual_hello};

mod services;

mod controllers;
use crate::controllers::test_mongo_controller;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("in main");
    test_mongo_controller::mongo_stuff().await;
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8087))?
    .run()
    .await
}
