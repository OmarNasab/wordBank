use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Phrase {
    pub id: i32,
    pub phrase: String,
    pub definition: String,
    pub note: String,
}