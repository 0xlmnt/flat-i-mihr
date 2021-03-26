-- Your SQL goes here

CREATE TABLE users
(
    id        uuid PRIMARY KEY,
    username  VARCHAR NOT NULL,
    firstname varchar not null,
    lastname  varchar,
    password  varchar not null,
    salt      varchar not null,
    email     varchar NOT NULL,
    wk_key    varchar,
    settings  varchar
)