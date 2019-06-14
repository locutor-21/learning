use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Move {
    x: i32,
    y: i32,
}

impl Move {
    pub fn new() -> Self {
        Move { x: 0, y: 0, }
    }
}
