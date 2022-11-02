use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use mongodb::bson::{doc, Document};
use mongodb::{options::ClientOptions, Client, Collection};
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

async fn new_mongo_stuff() {
    #[derive(Clone, Debug, Serialize, Deserialize)]
    struct Book {
        title: String,
        author: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    struct MongoService {
        collection: Collection<Document>,
    }

    impl MongoService {
        pub fn new(collection: Collection<Document>) -> MongoService {
            println!("in here ln 23");
            MongoService { collection }
        }

        pub fn create(&self, name: &str) -> Result<InsertOneResult, Error> {
            self.collection.insert_one(bson::doc! {"name": name}, None)
        }

        pub fn get(&self) -> Result<Option<bson::Document>, Error> {
            self.collection.find_one(bson::doc! {}, None)
        }
    }
}

async fn mongo_stuff() -> String {
    #[derive(Clone, Debug, Serialize, Deserialize)]
    struct Book {
        title: String,
        author: String,
    }

    // // Parse a connection string into an options struct.
    let _client_options = ClientOptions::parse("mongodb://localhost:8089").await;

    // Manually set an option.
    let mut client_options_okay = _client_options.unwrap();

    client_options_okay.app_name = Some("My App".to_string());

    let client_options_clone = client_options_okay.app_name.clone();
    match client_options_clone {
        Some(val) => println!("{}", val),
        None => println!("none value returned by client_options_clone"),
    }
    // Get a handle to the deployment.
    let _client = Client::with_options(client_options_okay);

    let client_clone = _client.clone();

    // List the names of the databases in that deployment.
    for db_name in _client.unwrap().list_database_names(None, None).await {
        println!("{:?}", db_name);
    }

    let db = client_clone.unwrap().database("mydb");

    // Get a handle to a collection of `Book`.
    let typed_collection = db.collection::<Book>("books");

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
    typed_collection.insert_many(books, None).await;

    // Query the books in the collection with a filter and an option.
    let filter = doc! { "author": "John Steinbeck" };
    // let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
    let mut cursor = typed_collection.find_one(filter, None).await;
    let res = cursor.unwrap().unwrap();
    println!("{:?}", res.title);
    res.title
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
