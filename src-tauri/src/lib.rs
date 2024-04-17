pub mod model;
pub mod schema;
pub mod commands;


use std::fs;
use diesel::{Connection, SqliteConnection};



pub fn establish_connection() -> SqliteConnection {
    let mut database_url = dirs::document_dir().expect("Could not get document directory");

    // Append "wordbank" to the path
    database_url.push("wordbank");

    // Check if the "wordbank" directory exists
    if !database_url.exists() {
        // Create the "wordbank" directory
        println!("{}", database_url.to_str().expect("Failed to convert path to string"));
        fs::create_dir_all(&database_url).expect("Failed to create wordbank directory");
    }
    database_url.push("wordbank.db");

    let database_url_str = database_url.to_str().expect("Failed to convert path to string");

    // Append "wordbank.db" to the path
     SqliteConnection::establish(&database_url_str)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url_str))
}