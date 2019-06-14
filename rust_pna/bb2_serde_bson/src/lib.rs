#[macro_use(bson, doc)] extern crate bson;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Move {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    x: i32, 
    y: i32,
}

impl Move {
    pub fn new() -> Self {
        Move { x: 0, y: 0 }
    }
}
