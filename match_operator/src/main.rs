#[derive(Debug)]
enum UsState {
    Ohio,
    Pennsylvania,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("Hello, world!");
    println!(
        "Nickel is {} cents",
        value_in_cents(Coin::Quarter(UsState::Ohio))
    );

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
