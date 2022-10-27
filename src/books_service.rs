use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::options::FindOptions;
use mongodb::{options::ClientOptions, Client};
use serde::{Deserialize, Serialize};

use mongodb::{error::Error, results::InsertOneResult, Collection};

// let client = Client::with_uri_str("mongodb://localhost:3009").await;
// let db = client.unwrap().database("mydb");

#[derive(Debug, Clone)]
pub struct BooksService {
    // client: Client::with_uri_str("mongodb://localhost:3009"),
    collection: Collection<Document>,
}

impl BooksService {
    pub fn new(collection: Collection<Document>) -> BooksService {
        BooksService { collection }
    }

    // pub fn create(&self, name: &str) -> Result<InsertOneResult, Error> {
    //     self.collection.insert_one()
    // }
}

// impl UserService {
//     pub fn new(collection: Collection<bson::Document>) -> UserService {
//         UserService { collection }
//     }

//     pub fn create(&self, name: &str) -> Result<InsertOneResult, Error> {
//         self.collection.insert_one(bson::doc! {"name": name}, None)
//     }

//     pub fn get(&self) -> Result<Option<bson::Document>, Error> {
//         self.collection.find_one(bson::doc! {}, None)
//     }
// }
