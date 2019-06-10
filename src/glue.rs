use crate::models::*;
use diesel::{prelude::*, PgConnection};

pub enum ItemType {
    Foo,
    Bar,
}

impl From<i32> for ItemType {
    fn from(type_num: i32) -> Self {
        match type_num {
            1 => ItemType::Foo,
            2 => ItemType::Bar,
            _ => panic!(format!("type num not support: {}", type_num)),
        }
    }
}

impl From<ItemType> for i32 {
    fn from(type_enum: ItemType) -> Self {
        match type_enum {
            ItemType::Foo => 1,
            ItemType::Bar => 2,
        }
    }
}

pub struct GluedItem {
    pub id: i32,
    pub instance_of: ItemType,
    pub title: String,
    pub number: Option<i32>,
    pub text: Option<String>,
}

trait ResolvableItem {
    fn resolve(self, db_conn: &PgConnection) -> GluedItem;
}

impl ResolvableItem for Item {
    fn resolve(self, db_conn: &PgConnection) -> GluedItem {
        match ItemType::from(self.instance_of) {
            ItemType::Foo => {
                use crate::schema::item_foos::dsl::*;

                let mut val: Vec<ItemFoo> = item_foos
                    .find(self.id)
                    .load::<ItemFoo>(db_conn)
                    .expect("Error loading item_foos");

                let item = val.pop().expect(&format!("No id: {}", self.id));

                GluedItem {
                    id: self.id,
                    instance_of: ItemType::Foo,
                    title: self.title,
                    number: None,
                    text: item.text,
                }
            }
            ItemType::Bar => {
                use crate::schema::item_bars::dsl::*;

                let mut val: Vec<ItemBar> = item_bars
                    .find(self.id)
                    .load::<ItemBar>(db_conn)
                    .expect("Error loading item_bars");

                let item = val.pop().expect(&format!("No id: {}", self.id));

                GluedItem {
                    id: self.id,
                    instance_of: ItemType::Foo,
                    title: self.title,
                    number: item.number,
                    text: None,
                }
            }
        }
    }
}

impl ResolvableItem for ItemFoo {
    fn resolve(self, db_conn: &PgConnection) -> GluedItem {
        use crate::schema::items::dsl::*;

        let mut val: Vec<Item> = items
            .find(self.id)
            .load::<Item>(db_conn)
            .expect("Error loading item_foo");

        let item = val.pop().expect(&format!("No id: {}", self.id));

        GluedItem {
            id: self.id,
            instance_of: ItemType::Foo,
            title: item.title,
            number: None,
            text: self.text,
        }
    }
}

impl ResolvableItem for ItemBar {
    fn resolve(self, db_conn: &PgConnection) -> GluedItem {
        use crate::schema::items::dsl::*;

        let mut val: Vec<Item> = items
            .find(self.id)
            .load::<Item>(db_conn)
            .expect("Error loading item_foo");

        let item = val.pop().expect(&format!("No id: {}", self.id));

        GluedItem {
            id: self.id,
            instance_of: ItemType::Foo,
            title: item.title,
            number: self.number,
            text: None,
        }
    }
}

pub enum TagType {
    Foo,
    Bar,
}

impl From<i32> for TagType {
    fn from(type_num: i32) -> Self {
        match type_num {
            1 => TagType::Foo,
            2 => TagType::Bar,
            _ => panic!(format!("type num not support: {}", type_num)),
        }
    }
}

impl From<TagType> for i32 {
    fn from(type_enum: TagType) -> Self {
        match type_enum {
            TagType::Foo => 1,
            TagType::Bar => 2,
        }
    }
}

pub struct GluedTag {
    pub id: i32,
    pub instance_of: TagType,
    pub title: String,
    pub number: Option<i32>,
    pub text: Option<String>,
}

pub trait ResolvableTag {
    fn resolve(self, db_conn: &PgConnection) -> GluedTag;
}

impl ResolvableTag for Tag {
    fn resolve(self, db_conn: &PgConnection) -> GluedTag {
        match TagType::from(self.instance_of) {
            TagType::Foo => {
                use crate::schema::tag_foos::dsl::*;

                let mut val: Vec<TagFoo> = tag_foos
                    .find(self.id)
                    .load::<TagFoo>(db_conn)
                    .expect("Error loading tag_foos");

                let tag = val.pop().expect(&format!("No id: {}", self.id));

                GluedTag {
                    id: self.id,
                    instance_of: TagType::Foo,
                    title: self.title,
                    number: None,
                    text: tag.text,
                }
            }
            TagType::Bar => {
                use crate::schema::tag_bars::dsl::*;

                let mut val: Vec<TagBar> = tag_bars
                    .find(self.id)
                    .load::<TagBar>(db_conn)
                    .expect("Error loading tag_bars");

                let tag = val.pop().expect(&format!("No id: {}", self.id));

                GluedTag {
                    id: self.id,
                    instance_of: TagType::Foo,
                    title: self.title,
                    number: tag.number,
                    text: None,
                }
            }
        }
    }
}

impl ResolvableTag for TagFoo {
    fn resolve(self, db_conn: &PgConnection) -> GluedTag {
        use crate::schema::tags::dsl::*;

        let mut val: Vec<Tag> = tags
            .find(self.id)
            .load::<Tag>(db_conn)
            .expect("Error loading tag_foo");

        let tag = val.pop().expect(&format!("No id: {}", self.id));

        GluedTag {
            id: self.id,
            instance_of: TagType::Foo,
            title: tag.title,
            number: None,
            text: self.text,
        }
    }
}

impl ResolvableTag for TagBar {
    fn resolve(self, db_conn: &PgConnection) -> GluedTag {
        use crate::schema::tags::dsl::*;

        let mut val: Vec<Tag> = tags
            .find(self.id)
            .load::<Tag>(db_conn)
            .expect("Error loading tag_foo");

        let tag = val.pop().expect(&format!("No id: {}", self.id));

        GluedTag {
            id: self.id,
            instance_of: TagType::Foo,
            title: tag.title,
            number: self.number,
            text: None,
        }
    }
}