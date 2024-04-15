// @generated automatically by Diesel CLI.

diesel::table! {
    medical_phrases (id) {
        id -> Nullable<Integer>,
        phrase -> Text,
        definition -> Text,
        note -> Text,
    }
}

diesel::table! {
    nouns (id) {
        id -> Nullable<Integer>,
        article -> Text,
        noun -> Text,
        plural -> Text,
        definition -> Text,
    }
}

diesel::table! {
    phrases (id) {
        id -> Nullable<Integer>,
        phrase -> Text,
        definition -> Text,
        note -> Text,
    }
}

diesel::table! {
    prefixes (id) {
        id -> Nullable<Integer>,
        verb_id -> Nullable<Integer>,
        prefix -> Nullable<Text>,
        definition -> Nullable<Text>,
    }
}

diesel::table! {
    propositions (id) {
        id -> Nullable<Integer>,
        verb_id -> Nullable<Integer>,
        proposition -> Nullable<Text>,
        definition -> Nullable<Text>,
    }
}

diesel::table! {
    verbs (id) {
        id -> Nullable<Integer>,
        verb -> Text,
        past_simple -> Text,
        past_perfect -> Text,
        definition -> Text,
        note -> Nullable<Text>,
    }
}

diesel::joinable!(prefixes -> verbs (verb_id));
diesel::joinable!(propositions -> verbs (verb_id));

diesel::allow_tables_to_appear_in_same_query!(
    medical_phrases,
    nouns,
    phrases,
    prefixes,
    propositions,
    verbs,
);
