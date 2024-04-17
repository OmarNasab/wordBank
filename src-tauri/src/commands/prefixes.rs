use diesel::prelude::*;
use crate::establish_connection;
use crate::model::{NewPrefix, Prefix};
use crate::schema::prefixes::dsl::prefixes;
use crate::schema::prefixes::verb_id;

#[tauri::command]
pub fn add_prefix(new_prefix: NewPrefix) -> Prefix{
    let connection = &mut establish_connection();
    use crate::schema::prefixes;
    diesel::insert_into(prefixes::table)
        .values(&new_prefix)
        .returning(Prefix::as_returning())
        .get_result(connection)
        .expect("Error saving new post")
}

#[tauri::command]
pub fn get_prefixes(verb: i32) -> Vec<Prefix>{
    let connection = &mut establish_connection();

    let results = prefixes
        .filter(verb_id.eq(verb))
        .select(Prefix::as_select())
        .load(connection)
        .expect("Error loading posts");
    results
}

pub fn search_prefixes(query: String) -> Vec<Prefix> {
    let connection = &mut establish_connection();
    use crate::schema::prefixes;
    let results = prefixes
        .filter(
            prefixes::prefix.like(format!("%{}%", query.clone()))
                .or(prefixes::definition.like(format!("%{}%", query)))
        )
        .load::<Prefix>(connection)
        .expect("Error loading prefixes");
    results
}