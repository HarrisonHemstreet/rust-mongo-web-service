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

async fn new_mongo_stuff() {
    #[derive(Clone, Debug, Serialize, Deserialize)]
    struct Book {
        title: String,
        author: String,
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

#[derive(Clone, Debug)]
struct MongoService {
    collection: Collection<Document>,
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

    let mut cursor = MongoService::get_all(&mongo_service_collection)
        .await
        .unwrap();

    while let Some(doc) = &cursor.next().await {
        println!("{:?}", doc)
    }

    // let res: String = cursor.await.unwrap();
    // TryStream uses try_collect() and collects into a Result<Vec<T>>
    // let v: <Cursor<Document>> = cursor.await.unwrap();
    // println!("{:?}", res.title);
    // res.title
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("in main");
    // mongo_stuff().await;
    test().await;
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
