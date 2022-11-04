mod services;
// mod services::mongo_service;
// use services::mongo_service::MongoService;
// use services::mongo_service;
use services::mongo_service::MongoService;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use mongodb::bson::doc;
use mongodb::Client;
use serde::{Deserialize, Serialize};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/mongotest")]
async fn mongo_test() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}

async fn mongo_stuff() -> String {
    let client = Client::with_uri_str("mongodb://localhost:8089")
        .await
        .unwrap();

    let typed_collection = client.database("mydb").collection("books");

    let books_collection = MongoService::new(typed_collection);

    books_collection.create_test().await.unwrap();

    let mut retrieve_all = books_collection.get_all().await.unwrap();

    while retrieve_all.advance().await.unwrap() {
        println!("{:?}", retrieve_all.deserialize_current().unwrap());
    }

    String::from("hi")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("in main");
    mongo_stuff().await;
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
