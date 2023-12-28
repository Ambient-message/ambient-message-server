use application::mappers::db_mapper::DbMapper;
use db::models::ChatModel;
use domain::chat_entity::ChatEntity;

pub struct ChatDbMapper {}

impl DbMapper<ChatEntity, ChatModel> for ChatDbMapper {
    fn to_db(entity: ChatEntity) -> ChatModel {
        ChatModel {
            id: entity.id,
            created: entity.created
        }
    }

    fn to_entity(model: ChatModel) -> ChatEntity {
        ChatEntity {
            id: model.id,
            created: model.created
        }
    }
}