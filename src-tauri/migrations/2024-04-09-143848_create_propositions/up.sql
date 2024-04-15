CREATE TABLE propositions
(
    id          INTEGER PRIMARY KEY NOT NULL,
    verb_id     INTEGER NOT NULL,
    proposition VARCHAR NOT NULL,
    definition  VARCHAR NOT NULL,
    FOREIGN KEY (verb_id) REFERENCES verbs (id)
)
