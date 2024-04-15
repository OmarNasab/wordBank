-- Your SQL goes here
CREATE TABLE verbs
(
    id           INTEGER PRIMARY KEY,
    verb         VARCHAR NOT NULL,
    past_simple  VARCHAR NOT NULL,
    past_perfect VARCHAR NOT NULL,
    definition   VARCHAR NOT NULL,
    note         VARCHAR
)