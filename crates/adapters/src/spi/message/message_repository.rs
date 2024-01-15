use actix_web::http::StatusCode;
use futures::{StreamExt, TryStreamExt};
use mongodb::bson::doc;
use mongodb::Collection;
use mongodb::options::FindOptions;
use uuid::Uuid;
use mongodb::Cursor;
use mongodb::bson::Document;

use application::mappers::db_mapper::DbMapper;
use application::repositories::message_repository_abstract::MessageRepositoryAbstract;
use domain::api_error::ApiError;
use domain::message_entity::MessageEntity;
use mongodb_db::{message_model::MessageModel, mongodb_connection::MongodbConnection};

use crate::spi::message::message_db_mapper::MessageDbMapper;

pub struct MessageRepository {
    pub mongodb: MongodbConnection,
}

impl MessageRepositoryAbstract for MessageRepository {
    async fn save(&self, message: &MessageEntity) -> Result<(), ApiError> {
        let client = self.mongodb.get_client();

        let db = client.database("Ambient");
        let collection: Collection<MessageModel> = db.collection("messages");

        let message = MessageDbMapper::to_db(message.clone());
        collection.insert_one(message, None).await
            .map_err(|err| ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Can't insert message",
                err,
            ))?;

        Ok(())
    }

    async fn get_messages(&self, chat_id: &Uuid) -> Result<Vec<MessageEntity>, ApiError> {
        let client = self.mongodb.get_client();

        let db = client.database("Ambient");
        let collection: Collection<MessageModel> = db.collection("messages");

        let filter = doc! { "chat_id": chat_id };

        let find_options = FindOptions::builder().build();
        let cursor = collection.find(filter, find_options).await
            .map_err(|err| ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Can't find messages",
                err,
            ))?;

        let documents: Vec<MessageModel> = cursor.try_collect().await
            .map_err(|err| ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Can't collect messages",
                err,
            ))?;

        Ok(documents.iter().map(|m| MessageDbMapper::to_entity(m.clone())).collect())
    }
}