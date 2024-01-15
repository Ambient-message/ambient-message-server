use mongodb::Client;
use mongodb::options::ClientOptions;

pub struct MongodbConnection {
    client: Client,
}

impl MongodbConnection {
    pub async fn new(connection_string: &str) -> Self {
        let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
        let client = Client::with_options(client_options).unwrap();

        Self { client }
    }

    pub fn get_client(&self) -> &Client {
        &self.client
    }
}