use application::mappers::db_mapper::DbMapper;
use db::models::UserModel;
use domain::message_entity::MessageEntity;
use domain::user_entity::UserEntity;
use mongodb_db::message_model::MessageModel;

pub struct MessageDbMapper {}

impl DbMapper<MessageEntity, MessageModel> for MessageDbMapper {
    fn to_db(entity: MessageEntity) -> MessageModel {
        MessageModel {
            _id: entity.id,
            sender_id: entity.sender_id,
            chat_id: entity.chat_id,
            text: entity.text,
        }
    }

    fn to_entity(model: MessageModel) -> MessageEntity {
        MessageEntity {
            id: model._id,
            sender_id: model.sender_id,
            chat_id: model.chat_id,
            text: model.text,
        }
    }
}