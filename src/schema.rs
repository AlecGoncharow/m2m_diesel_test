table! {
    item_bars (id) {
        id -> Int4,
        number -> Nullable<Int4>,
    }
}

table! {
    item_foos (id) {
        id -> Int4,
        text -> Nullable<Text>,
    }
}

table! {
    items (id) {
        id -> Int4,
        instance_of -> Int4,
        title -> Text,
    }
}

table! {
    item_tags (item_id, tag_id) {
        item_id -> Int4,
        tag_id -> Int4,
        value -> Int4,
    }
}

table! {
    tag_bars (id) {
        id -> Int4,
        number -> Nullable<Int4>,
    }
}

table! {
    tag_foos (id) {
        id -> Int4,
        text -> Nullable<Text>,
    }
}

table! {
    tags (id) {
        id -> Int4,
        instance_of -> Int4,
        title -> Text,
    }
}

joinable!(item_tags -> items (item_id));
joinable!(item_tags -> tags (tag_id));

allow_tables_to_appear_in_same_query!(
    item_bars, item_foos, items, item_tags, tag_bars, tag_foos, tags,
);
