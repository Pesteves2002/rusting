#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
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
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(UsState) => {
            println!("State quarter from {:?}!", UsState);
            25
        }
    }
}

fn add_one(coin: Option<i32>) -> Option<i32> {
    match coin {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}

fn main() {
    let coin = Coin::Penny;
    println!("{}", value_in_cents(coin));

    let five = Some(5);
    let six = add_one(five);
    let none = add_one(None);

    if let Some(max) = six {
        println!("six: {}", max);
    }
}
