table! {
    item (id) {
        id -> Int4,
        instance_of -> Int4,
        title -> Text,
    }
}

table! {
    item_bar (id) {
        id -> Int4,
        number -> Nullable<Int4>,
    }
}

table! {
    item_foo (id) {
        id -> Int4,
        text -> Nullable<Text>,
    }
}

table! {
    item_tag (item_id, tag_id) {
        item_id -> Int4,
        tag_id -> Int4,
        value -> Int4,
    }
}

table! {
    tag (id) {
        id -> Int4,
        instance_of -> Int4,
        title -> Text,
    }
}

table! {
    tag_bar (id) {
        id -> Int4,
        number -> Nullable<Int4>,
    }
}

table! {
    tag_foo (id) {
        id -> Int4,
        text -> Nullable<Text>,
    }
}

joinable!(item_tag -> item (item_id));
joinable!(item_tag -> tag (tag_id));

allow_tables_to_appear_in_same_query!(
    item,
    item_bar,
    item_foo,
    item_tag,
    tag,
    tag_bar,
    tag_foo,
);
