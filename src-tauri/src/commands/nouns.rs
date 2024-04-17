use diesel::prelude::*;
use crate::establish_connection;
use crate::model::{Noun, NewNoun};
use crate::schema::nouns::dsl::nouns;

#[tauri::command]
pub fn add_noun(new_noun: NewNoun) -> Noun{
    let connection = &mut establish_connection();
    use crate::schema::nouns;
    diesel::insert_into(nouns::table)
        .values(&new_noun)
        .returning(Noun::as_returning())
        .get_result(connection)
        .expect("Error saving new post")
}

#[tauri::command]
pub fn get_nouns() -> Vec<Noun>{
    let connection = &mut establish_connection();
    let results = nouns
        .select(Noun::as_select())
        .load(connection)
        .expect("Error loading posts");
    results
}

pub fn search_nouns(query: String) -> Vec<NewNoun> {
    let connection = &mut establish_connection();
    use crate::schema::nouns;
    let results = nouns
        .filter(
            nouns::noun.like(format!("%{}%", query.clone()))
                .or(nouns::definition.like(format!("%{}%", query.clone())))
                .or(nouns::article.like(format!("%{}%", query.clone())))
                .or(nouns::plural.like(format!("%{}%", query)))
        )
        .select(NewNoun::as_select())
        .load::<NewNoun>(connection)
        .expect("Error loading nouns");
    results
}