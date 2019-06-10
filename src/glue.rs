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
                    .expect("Error loading item_foo");

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
                    .expect("Error loading item_foo");

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
