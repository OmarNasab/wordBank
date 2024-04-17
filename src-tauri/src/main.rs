// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use app::establish_connection;
use app::commands::verbs::*;
use app::__cmd__get_verbs;
use app::__cmd__add_verb;
use app::__cmd__get_verb;
use app::commands::medical_phrases::*;
use app::__cmd__add_medical_phrase;
use app::__cmd__get_medical_phrases;
use app::commands::phrases::*;
use app::__cmd__add_phrase;
use app::__cmd__get_phrases;
use app::commands::nouns::*;
use app::__cmd__add_noun;
use app::__cmd__get_nouns;
use app::commands::propositions::*;
use app::__cmd__add_proposition;
use app::__cmd__get_propositions;
use app::commands::prefixes::*;
use app::__cmd__add_prefix;
use app::__cmd__get_prefixes;
use app::commands::search::*;
use app::__cmd__search_all;
use std::error::Error;
use diesel::sqlite::Sqlite;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();


pub fn run_migrations(connection: &mut impl MigrationHarness<Sqlite>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {

    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}
fn main() {
        let connection= &mut establish_connection();
    run_migrations(connection);
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            add_verb,
            get_verbs,
            get_verb,
            add_medical_phrase,
            get_medical_phrases,
            add_phrase,
            get_phrases,
            add_noun,
            get_nouns,
            add_proposition,
            get_propositions,
            add_prefix,
            get_prefixes,
            search_all
      ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

