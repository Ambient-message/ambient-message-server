-- Your SQL goes here
CREATE TABLE IF NOT EXISTS userchats
(
    id uuid NOT NULL,
    user_model_id uuid NOT NULL,
    chat_model_id uuid NOT NULL,
    foreign key (user_model_id) references users(id),
    foreign key (chat_model_id) references chats(id),
    CONSTRAINT userchats_pkey PRIMARY KEY(id)
)