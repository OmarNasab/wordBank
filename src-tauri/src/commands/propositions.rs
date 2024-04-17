use diesel::prelude::*;
use crate::establish_connection;
use crate::model::{Proposition, NewProposition};
use crate::schema::propositions::dsl::propositions;
use crate::schema::propositions::verb_id;

#[tauri::command]
pub fn add_proposition(new_proposition: NewProposition) -> Proposition{
    let connection = &mut establish_connection();
    use crate::schema::propositions;
    diesel::insert_into(propositions::table)
        .values(&new_proposition)
        .returning(Proposition::as_returning())
        .get_result(connection)
        .expect("Error saving new post")
}

#[tauri::command]
pub fn get_propositions(verb: i32) -> Vec<Proposition>{
    let connection = &mut establish_connection();
    let results = propositions
        .filter(verb_id.eq(verb))
        .select(Proposition::as_select())
        .load(connection)
        .expect("Error loading posts");
    results
}

pub fn search_propositions(query: String) -> Vec<Proposition> {
    let connection = &mut establish_connection();
    use crate::schema::propositions;
    let results = propositions
        .filter(
            propositions::proposition.like(format!("%{}%", query.clone()))
                .or(propositions::definition.like(format!("%{}%", query)))
        )
        .load::<Proposition>(connection)
        .expect("Error loading propositions");
    results
}