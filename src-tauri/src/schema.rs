table! {
    tasks (id) {
        id -> Integer,
        title -> Text,
        task_type -> Text,
        tags -> Nullable<Text>,
        body -> Text,
        done -> Bool,
        created_at -> Timestamp,
        due_at -> Timestamp,
    }
}
