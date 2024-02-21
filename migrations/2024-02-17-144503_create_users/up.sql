CREATE TABLE IF NOT EXISTS users
(
    id          uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    email       text NOT NULL UNIQUE
);
