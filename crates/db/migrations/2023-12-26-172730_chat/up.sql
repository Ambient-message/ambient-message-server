-- Your SQL goes here
CREATE TABLE IF NOT EXISTS chats
(
    id uuid NOT NULL,
    CONSTRAINT chats_pkey PRIMARY KEY(id)
)