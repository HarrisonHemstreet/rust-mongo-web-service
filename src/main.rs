mod books_service;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::io::Result;
// use actix_web::{get, web, App, HttpServer, };
use mongodb::bson::{doc, Document};
use mongodb::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Record {
    my_json: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn wut_up() -> impl Responder {
    HttpResponse::Ok().body("wut up!")
}

#[post("/json")]
async fn my_json(myres: web::Json<Record>) -> impl Responder {
    HttpResponse::Ok().body(format!("howdy {}!", myres.my_json))
    // Ok(format!("Welcome {}!", myres.my_json))
}

// #[get("/")]
// async fn index(data: web::Data<AppState>) -> String {
//     let app_name = &data.app_name; // <- get app_name
//     format!("Hello {app_name}!") // <- response with app_name
// }

// fn execute_mongo() {
//     let _client_options = ClientOptions::parse("mongodb://localhost:3009").await;
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// struct Book {
//     title: String,
//     author: String,
// }

// fn what_am_i(input: Collection<Document>) -> Collection<Document> {
//     input
// }

pub struct BooksContainer {
    books: books_service::BooksService,
}

impl BooksContainer {
    pub fn new(books: books_service::BooksService) -> Self {
        BooksContainer { books }
    }
}

pub struct AppState {
    // app_name: String,
    books_container: BooksContainer,
}

#[actix_web::main]
async fn main() -> Result<()> {
    let client = Client::with_uri_str("mongodb://localhost:3009").await;
    let db = client.unwrap().database("mydb");

    let books_collection = db.collection::<Document>("books");
    // mongodb::Collection<mongodb::bson::Document>

    HttpServer::new(move || {
        let books_container =
            BooksContainer::new(books_service::BooksService::new(books_collection.clone()));

        App::new()
            .app_data(AppState { books_container })
            .service(hello)
            .service(echo)
            .service(my_json)
            .route("/hey", web::get().to(manual_hello))
            .route("/wutup", web::get().to(wut_up))
    })
    .bind(("127.0.0.1", 3080))?
    .run()
    .await
}
