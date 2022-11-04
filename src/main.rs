use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
// use futures_util::stream::try_stream::TryStreamExt;
use futures::stream::StreamExt;
use mongodb::bson::{doc, Document};
use mongodb::{error::Error, results::InsertOneResult};
use mongodb::{options::ClientOptions, Client, Collection, Cursor};
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

#[derive(Clone, Debug)]
struct MongoService {
    collection: Collection<Document>,
    // collection: Book,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}

impl MongoService {
    pub fn new(collection: Collection<Document>) -> MongoService {
        println!("in here ln 23");
        MongoService { collection }
    }

    pub async fn create(&self, name: &str) -> Result<InsertOneResult, mongodb::error::Error> {
        self.collection.insert_one(doc! {"name": name}, None).await
    }

    pub async fn create_test(&self) -> Result<InsertOneResult, mongodb::error::Error> {
        self.collection
            .insert_one(
                doc! {"title": "This is a test".to_string(),
                "author": "This is a test".to_string()},
                None,
            )
            .await
        // self.collection.insert_one(doc! {"name": name}, None).await
    }

    pub async fn get(&self) -> Result<Option<Document>, Error> {
        self.collection.find_one(doc! {}, None).await
    }

    pub async fn get_all(&self) -> Result<Cursor<Document>, Error> {
        self.collection.find(None, None).await
    }
}

async fn mongo_stuff() -> String {
    let client = Client::with_uri_str("mongodb://localhost:8089")
        .await
        .unwrap();

    // Get a handle to a collection of `Book`.
    let typed_collection = client.database("mydb").collection("books");

    // let books_collection = MongoService::new(typed_collection);

    let books = vec![
        Book {
            title: "The Grapes of Wrath".to_string(),
            author: "John Steinbeck".to_string(),
        },
        Book {
            title: "To Kill a Mockingbird".to_string(),
            author: "Harper Lee".to_string(),
        },
    ];

    // Insert the books into "mydb.books" collection, no manual conversion to BSON necessary.
    typed_collection.insert_many(books, None).await.unwrap();

    // Query the books in the collection with a filter and an option.
    let filter = doc! { "author": "John Steinbeck" };
    // let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
    let mut cursor = typed_collection.find(filter, None).await.unwrap();
    while cursor.advance().await.unwrap() {
        println!("{:?}", cursor.deserialize_current().unwrap());
    }
    String::from("hi")
}

async fn test() {
    println!("in here");
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();
    let database = client.database("mydb");
    let collection = database.collection("books");

    // let service_container =
    //     ServiceContainer::new(service::UserService::new(user_collection.clone()));
    // let books_collection = db.collection("books");
    let mongo_service_collection = MongoService::new(collection);
    MongoService::create_test(&mongo_service_collection)
        .await
        .unwrap();

    let mut cursor_result = MongoService::get_all(&mongo_service_collection).await;
    let mut cursor = match cursor_result {
        Ok(c) => c,
        Err(e) => panic!("cursor error: {:?}", e),
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("in main");
    mongo_stuff().await;
    // test().await;
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
