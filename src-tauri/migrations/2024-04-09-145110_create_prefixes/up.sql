CREATE TABLE prefixes
(
    id          INTEGER PRIMARY KEY NOT NULL,
    verb_id     INTEGER NOT NULL,
    prefix      VARCHAR NOT NULL ,
    definition  VARCHAR NOT NULL ,
    FOREIGN KEY (verb_id) REFERENCES verbs (id)
)