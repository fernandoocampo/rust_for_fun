#[derive(Debug)]
pub enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Kentucky,
}

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

pub fn plus_one(value: Option<i32>) -> Option<i32> {
    // this block can be replaced for a simpler one.
    // match value {
    //     None => None,
    //     Some(i) => Some(i + 1),
    // }
    value.map(|i| i + 1)
}
