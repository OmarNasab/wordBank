// @generated automatically by Diesel CLI.

diesel::table! {
    medical_phrases (id) {
        id -> Integer,
        phrase -> Text,
        definition -> Text,
        note -> Nullable<Text>,
    }
}

diesel::table! {
    nouns (id) {
        id -> Integer,
        article -> Text,
        noun -> Text,
        plural -> Text,
        definition -> Text,
    }
}

diesel::table! {
    phrases (id) {
        id -> Integer,
        phrase -> Text,
        definition -> Text,
        note -> Nullable<Text>,
    }
}

diesel::table! {
    prefixes (id) {
        id -> Integer,
        verb_id -> Integer,
        prefix -> Text,
        definition -> Text,
    }
}

diesel::table! {
    propositions (id) {
        id -> Integer,
        verb_id -> Integer,
        proposition -> Text,
        definition -> Text,
    }
}

diesel::table! {
    verbs (id) {
        id -> Integer,
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
