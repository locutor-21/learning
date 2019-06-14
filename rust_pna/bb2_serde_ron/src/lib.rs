use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Move {
    x: i32, 
    y: i32,
}

impl Move {
    pub fn new() -> Self {
        Move { x: 0, y: 0 }
    }
}
