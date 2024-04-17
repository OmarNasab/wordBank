use diesel::prelude::*;
use crate::establish_connection;
use crate::model::{Phrase, NewPhrase};
use crate::schema::phrases::dsl::phrases;

#[tauri::command]
pub fn add_phrase(new_phrase: NewPhrase) -> Phrase{
    let connection = &mut establish_connection();
    use crate::schema::phrases;
    diesel::insert_into(phrases::table)
        .values(&new_phrase)
        .returning(Phrase::as_returning())
        .get_result(connection)
        .expect("Error saving new post")
}

#[tauri::command]
pub fn get_phrases() -> Vec<Phrase>{
    let connection = &mut establish_connection();
    let results = phrases
        .select(Phrase::as_select())
        .load(connection)
        .expect("Error loading posts");
    results
}

pub fn search_phrases(query: String) -> Vec<NewPhrase> {
    let connection = &mut establish_connection();
    use crate::schema::phrases;
    let results = phrases
        .filter(
            phrases::phrase.like(format!("%{}%", query.clone()))
                .or(phrases::definition.like(format!("%{}%", query.clone())))
                .or(phrases::note.like(format!("%{}%", query)))
        )
        .select(NewPhrase::as_select())
        .load::<NewPhrase>(connection)
        .expect("Error loading phrases");
    results
}