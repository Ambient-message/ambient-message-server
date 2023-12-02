-- Your SQL goes here
CREATE TABLE Users (
  Id SERIAL PRIMARY KEY,
  UserName TEXT NOT NULL,
  Password TEXT NOT NULL
)