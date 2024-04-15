CREATE TABLE prefixes
(
    id          INTEGER PRIMARY KEY,
    verb_id     INTEGER,
    prefix VARCHAR,
    definition  VARCHAR,
    FOREIGN KEY (verb_id) REFERENCES verbs (id)
)