// @generated automatically by Diesel CLI.

diesel::table! {
    chats (id) {
        id -> Uuid,
        created -> Timestamptz,
    }
}

diesel::table! {
    userchats (id) {
        id -> Uuid,
        user_model_id -> Uuid,
        chat_model_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        password -> Text,
    }
}

diesel::joinable!(userchats -> chats (chat_model_id));
diesel::joinable!(userchats -> users (user_model_id));

diesel::allow_tables_to_appear_in_same_query!(
    chats,
    userchats,
    users,
);
