-- Your SQL goes here

-- users -------
---------------
---------------
CREATE TABLE users (
    id         SERIAL PRIMARY KEY,
    first_name VARCHAR(100)  NOT NULL,
    last_name  VARCHAR(100)  NOT NULL,
    phone      VARCHAR(12)   NOT NULL,
    email      VARCHAR(100)  NOT NULL,
    password   VARCHAR(1000) NOT NULL,
    perm       SMALLINT NOT NULL,

    UNIQUE(phone),
    UNIQUE(email)
);
