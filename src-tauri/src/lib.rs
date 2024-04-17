pub mod model;
pub mod schema;
pub mod commands;


use dotenvy::dotenv;
use std::env;
use diesel::{Connection, SqliteConnection};



pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url ="database_url";
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}