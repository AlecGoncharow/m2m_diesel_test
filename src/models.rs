use crate::schema::*;

#[derive(Queryable, Debug)]
pub struct Item {
    pub id: i32,
    pub instance_of: i32,
    pub title: String
}

#[derive(Insertable, Debug)]
#[table_name="items"]
pub struct NewItem<'a> {
    pub instance_of: i32,
    pub title: &'a str,
}

#[derive(Queryable, Insertable, Debug)]
pub struct ItemBar {
    pub id: i32,
    pub number: Option<i32>,
}

#[derive(Queryable, Insertable, Debug)]
pub struct ItemFoo {
    pub id: i32,
    pub text: Option<String>
}

#[derive(Queryable, Insertable, Debug)]
pub struct ItemTag {
    item_id: i32,
    tag_id: i32,
    value: i32,
}

#[derive(Queryable, Debug)]
pub struct Tag {
    pub id: i32,
    pub instance_of: i32,
    pub title: String
}

#[derive(Insertable, Debug)]
#[table_name="tags"]
pub struct NewTag<'a> {
    pub instance_of: i32,
    pub title: &'a str,
}

#[derive(Queryable, Insertable, Debug)]
pub struct TagBar {
    pub id: i32,
    pub number: Option<i32>,
}

#[derive(Queryable, Insertable, Debug)]
pub struct TagFoo {
    pub id: i32,
    pub text: Option<String>
}

