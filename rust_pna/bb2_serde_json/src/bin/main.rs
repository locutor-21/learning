use bb2_serde_json::Move;
use serde_json::Result;
use std::str;

pub fn main() -> Result<()> {
    let a = Move::new();

    let v = serde_json::to_vec(&a)?;
    
    let b: Move = serde_json::from_slice(&v[..])?;

    println!("a: {:?}", a);
    println!("v: {:?}", v);
    println!("b: {:?}", b);

    let j = str::from_utf8(&v).unwrap();

    println!("j: {:?}", j);

    Ok(())
}
