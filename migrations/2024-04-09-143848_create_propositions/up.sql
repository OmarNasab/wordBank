CREATE TABLE propositions
(
    id          INTEGER PRIMARY KEY,
    verb_id     INTEGER,
    proposition VARCHAR,
    definition  VARCHAR,
    FOREIGN KEY (verb_id) REFERENCES verbs (id)
)
