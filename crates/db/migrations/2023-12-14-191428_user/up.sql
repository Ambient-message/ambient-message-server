-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users
(
    id       uuid                              NOT NULL UNIQUE,
    username text COLLATE pg_catalog."default" NOT NULL UNIQUE,
    password text COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT users_pkey PRIMARY KEY (id)
)