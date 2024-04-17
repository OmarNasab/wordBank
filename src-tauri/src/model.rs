use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::phrases)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Phrase {
    pub id: i32,
    pub phrase: String,
    pub definition: String,
    pub note: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::phrases)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]

pub struct NewPhrase{
    pub phrase: String,
    pub definition: String,
    pub note: Option<String>
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::nouns)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Noun {
    pub id: i32,
    pub article: String,
    pub noun: String,
    pub plural: String,
    pub definition: String
}
#[derive(Insertable, Serialize, Deserialize, Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::nouns)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewNoun {
    pub article: String,
    pub noun: String,
    pub plural: String,
    pub definition: String
}


#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::verbs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Verb{
    pub id: i32,
    pub verb: String,
    pub past_simple: String,
    pub past_perfect: String,
    pub definition: String,
    pub note: Option<String>
}

#[derive(Insertable, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::verbs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]

pub struct NewVerb {
    pub verb: String,
    pub past_simple: String,
    pub past_perfect: String,
    pub definition: String,
    pub note: Option<String>
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::medical_phrases)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct MedicalPhrase{
    pub id: i32,
    pub phrase: String,
    pub definition: String,
    pub note: Option<String>
}

#[derive(Insertable, Serialize, Deserialize, Selectable, Queryable)]
#[diesel(table_name = crate::schema::medical_phrases)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]

pub struct NewMedicalPhrase{
    pub phrase: String,
    pub definition: String,
    pub note: Option<String>
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::propositions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Proposition{
    pub id: i32,
    pub verb_id: i32,
    pub proposition: String,
    pub definition: String
}
#[derive(Insertable, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::propositions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]

pub struct NewProposition{
    pub verb_id: i32,
    pub proposition: String,
    pub definition: String
}
#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::prefixes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Prefix{
    pub id: i32,
    pub verb_id: i32,
    pub prefix: String,
    pub definition: String
}

#[derive(Insertable, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::prefixes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]

pub struct NewPrefix{
    pub verb_id: i32,
    pub prefix: String,
    pub definition: String
}