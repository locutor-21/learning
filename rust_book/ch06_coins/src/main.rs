#![allow(dead_code)] //so we don't get notifications about unused code
#[derive(Debug)] //so we can inspect the state in a minute
enum UsState {
    Alabama, 
    Alaska, 
    NewYork,
    Illinois, 
    Massachussets,
}

enum Coin {
    Penny, 
    Nickel,
    Dime, 
    Quarter(UsState), 
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}
