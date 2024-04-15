use diesel::dsl::Nullable;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::phrases)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Phrase {
    pub id: i32,
    pub phrase: String,
    pub definition: String,
    pub note: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::nouns)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Noun {
    pub id: i32,
    pub article: String,
    pub noun: String,
    pub plural: String,
    pub definition: String
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::verbs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Verb{
    id: i32,
    verb: String,
    past_simple: String,
    past_perfect: String,
    definition: String,
    note: Option<String>
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::medical_phrases)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct MedicalPhrases{
    id: i32,
    phrase: String,
    definition: String,
    note: Option<String>
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::propositions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Proposition{
    id: i32,
    verb_id: i32,
    proposition: String,
    definition: String
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::prefixes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Prefix{
    id: i32,
    verb_id: i32,
    prefix: String,
    definition: String
}