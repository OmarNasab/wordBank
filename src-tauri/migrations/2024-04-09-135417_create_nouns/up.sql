-- Your SQL goes here
CREATE TABLE nouns
(
    id         INTEGER PRIMARY KEY NOT NULL,
    article    VARCHAR NOT NULL,
    noun       VARCHAR NOT NULL,
    plural     VARCHAR NOT NULL,
    definition VARCHAR NOT NULL
)