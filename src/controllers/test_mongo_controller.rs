use crate::services::mongo_service::MongoService;
use mongodb::Client;
pub async fn mongo_stuff() {
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
}
