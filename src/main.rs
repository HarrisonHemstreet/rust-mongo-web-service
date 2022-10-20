use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
// use actix_web::{get, web, App, HttpServer, };
use mongodb::bson::{doc, Document};
use mongodb::{options::ClientOptions, Client};
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

#[get("/json")]
async fn my_json(myres: web::Json<Record>) -> impl Responder {
    HttpResponse::Ok().body(format!("howdy {}!", myres.my_json))
    // Ok(format!("Welcome {}!", myres.my_json))
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

// fn execute_mongo() {
//     let _client_options = ClientOptions::parse("mongodb://localhost:3009").await;
// }

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}

pub struct AppState {
    app_name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Client::with_uri_str("mongodb://localhost:3009").await;
    let db = client.unwrap().database("mydb");

    let book_collection = db.collection::<Document>("books");

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
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
