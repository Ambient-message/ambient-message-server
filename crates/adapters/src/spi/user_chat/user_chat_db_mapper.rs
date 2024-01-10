use application::mappers::db_mapper::DbMapper;
use db::models::UserChatModel;
use domain::user_chat_entity::UserChatEntity;

pub struct UserChatDbMapper {}

impl DbMapper<UserChatEntity, UserChatModel> for UserChatDbMapper {
    fn to_db(entity: UserChatEntity) -> UserChatModel {
        UserChatModel {
            id: entity.id,
            user_model_id: entity.user_id,
            chat_model_id: entity.chat_id,
        }
    }

    fn to_entity(model: UserChatModel) -> UserChatEntity {
        UserChatEntity {
            id: model.id,
            user_id: model.user_model_id,
            chat_id: model.chat_model_id,
        }
    }
}
