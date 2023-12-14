-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users
(
    id uuid NOT NULL,
    username text COLLATE pg_catalog."default" NOT NULL,
    password text COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT users_pkey PRIMARY KEY (id)
)
