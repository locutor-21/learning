use bb2_serde_ron::Move;
use ron::de::{self, Result};
use ron::ser;
use std::fs::{self, File};

pub fn main() -> Result<()> {
    let a = Move::new();

    fs::write("ron.txt", ser::to_string(&a).unwrap()).unwrap();
    
    let f = File::open("ron.txt").unwrap();

    let b: Move = de::from_reader(&f).unwrap();

    println!("{:?}", a);
    println!("{:?}", f);
    println!("{:?}", b);

    Ok(())
}
