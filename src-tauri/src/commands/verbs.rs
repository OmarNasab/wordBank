use diesel::prelude::*;
use crate::establish_connection;
use crate::model::{Verb, NewVerb};
use crate::schema::verbs::dsl::verbs;

#[tauri::command]
pub fn add_verb(new_verb: NewVerb) -> Verb{
    let connection = &mut establish_connection();
    use crate::schema::verbs;
    diesel::insert_into(verbs::table)
        .values(&new_verb)
        .returning(Verb::as_returning())
        .get_result(connection)
        .expect("Error saving new post")
}

#[tauri::command]
pub fn get_verbs() -> Vec<Verb>{
    let connection = &mut establish_connection();
    use crate::schema::verbs;
    let results = verbs
        .select(Verb::as_select())
        .order(verbs::id.desc())
        .load(connection)
        .expect("Error loading posts");
    results
}

#[tauri::command]
pub fn get_verb(id: i32) -> Verb{
    let connection = &mut establish_connection();
    let results = verbs
        .select(Verb::as_select())
        .find(id)
        .first(connection)
        .expect("Error loading posts");
    results
}

pub fn search_verbs(query: String) -> Vec<NewVerb> {
    let connection = &mut establish_connection();
    use crate::schema::verbs;
    let results = verbs
        .filter(
            verbs::verb.like(format!("%{}%", query.clone()))
                .or(verbs::past_simple.like(format!("%{}%", query.clone())))
                .or(verbs::past_perfect.like(format!("%{}%", query.clone())))
                .or(verbs::definition.like(format!("%{}%", query.clone())))
                .or(verbs::note.like(format!("%{}%", query)))
        )
        .select(NewVerb::as_select())
        .order(verbs::id.desc())
        .load::<NewVerb>(connection)
        .expect("Error loading verbs");
    results
}