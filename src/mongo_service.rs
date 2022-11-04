use mongodb::bson::{doc, Document};
use mongodb::{error::Error, results::InsertOneResult};
use mongodb::{options::ClientOptions, Client, Collection, Cursor};

#[derive(Clone, Debug)]
pub struct MongoService {
    collection: Collection<Document>,
    // collection: Book,
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
