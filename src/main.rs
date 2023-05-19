#[derive(Debug)]
enum useState {
    Pradesh1,
    Pradesh2,
    Pradesh3,
    Pradesh4,
    Pradesh5,
}

enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter(useState),
}

fn get_value_of_coin(coin: Coins) -> u8 {
    match coin {
        Coins::Penny => 1,
        Coins::Nickel => 5,
        Coins::Dime => 10,
        Coins::Quarter(state) => {
            println!("state is {:?}", state);
            25
        }
    }
}

fn main() {
    let new_coin = Coins::Quarter(useState::Pradesh1);
    let new_coin_value = get_value_of_coin(new_coin);
    println!("the coin value is {}", new_coin_value);
}
