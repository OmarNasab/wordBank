use diesel::prelude::*;
use crate::establish_connection;
use crate::model::{MedicalPhrase, NewMedicalPhrase};
use crate::schema::medical_phrases::dsl::medical_phrases;
#[tauri::command]
pub fn add_medical_phrase(new_medical_phrase: NewMedicalPhrase) -> MedicalPhrase{
    let connection = &mut establish_connection();
    use crate::schema::medical_phrases;
    diesel::insert_into(medical_phrases::table)
        .values(&new_medical_phrase)
        .returning(MedicalPhrase::as_returning())
        .get_result(connection)
        .expect("Error saving new post")
}

#[tauri::command]
pub fn get_medical_phrases() -> Vec<MedicalPhrase>{
    let connection = &mut establish_connection();
    use crate::schema::medical_phrases;
    let results = medical_phrases
        .select(MedicalPhrase::as_select())
        .order(medical_phrases::id.desc())
        .load(connection)
        .expect("Error loading posts");
    results
}

pub fn search_medical_phrases(query: String) -> Vec<NewMedicalPhrase> {
    let connection = &mut establish_connection();
    use crate::schema::medical_phrases;
    let results = medical_phrases
        .filter(
            medical_phrases::phrase.like(format!("%{}%", query.clone()))
                .or(medical_phrases::definition.like(format!("%{}%", query.clone())))
                .or(medical_phrases::note.like(format!("%{}%", query)))
        )
        .select(NewMedicalPhrase::as_select())
        .order(medical_phrases::id.desc())
        .load::<NewMedicalPhrase>(connection)
        .expect("Error loading medical phrases");
    results
}