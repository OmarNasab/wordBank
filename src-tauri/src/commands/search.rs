use crate::commands::medical_phrases::search_medical_phrases;
use crate::commands::nouns::search_nouns;
use crate::commands::phrases::search_phrases;
use crate::commands::prefixes::search_prefixes;
use crate::commands::propositions::search_propositions;
use crate::commands::verbs::search_verbs;
use serde_json::json;

#[tauri::command]
pub fn search_all(query: String) -> serde_json::Value {
    let mut results = json!({});

    let noun_results = search_nouns(query.clone());
    if  noun_results.len() > 0 {
        results["nouns"] = json!(noun_results);
    }

    let verb_results = search_verbs(query.clone());
    if  verb_results.len() > 0 {
        results["verbs"] = json!(verb_results);
    }

    let phrase_results = search_phrases(query.clone());
    if  phrase_results.len() > 0 {
        results["phrases"] = json!(phrase_results);
    }

    let medical_phrase_results = search_medical_phrases(query.clone());
    if  medical_phrase_results.len() > 0 {
        results["medical_phrases"] = json!(medical_phrase_results);
    }

    let proposition_results = search_propositions(query.clone());
    if  proposition_results.len() > 0 {
        results["propositions"] = json!(proposition_results);
    }

    let prefix_results = search_prefixes(query.clone());
    if  prefix_results.len() > 0 {
        results["prefixes"] = json!(prefix_results);
    }
    println!("{}", results);
    results
}