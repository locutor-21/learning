#![allow(unused_variables)]
#![allow(dead_code)]

use std::collections::HashMap;
use rand::Rng;

pub fn mean(v: &[i32]) -> f64 {
    let mut sum = 0;
    let mut count = 0;

    for i in v {
        sum += i;
        count += 1;
    }

    sum as f64 / count as f64
}

pub fn median(v: &mut [i32]) -> i32 {
    v.sort();
    let mid = v.len()/2;
    v[mid]
}

pub fn mode(v: &[i32]) -> i32 {
    let mut map = HashMap::new();

    for number in v {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }
    
    *map
        .into_iter()
        .max_by_key(|&(_, count)|count)
        .map(|(val, _)|val)
        .expect("Cannot compute the mode of zero numbers")
}

fn main() {
    let mut v = Vec::new();
    
    for _ in 0..20 {
        v.push(rand::thread_rng().gen_range(1, 11));
    }

    println!("mean:   {}", mean(&v));
    println!("median: {}", median(&mut v));
    println!("mode:   {}", mode(&v));
    println!("vector: {:?}", v);

}
