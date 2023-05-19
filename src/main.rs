#[derive(Debug)]
enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn get_value_of_coin(coin: Coins) -> u8 {
    match coin {
        Coins::Penny => 1,
        Coins::Nickel => 5,
        Coins::Dime => 10,
        Coins::Quarter => 25,
    }
}

fn main() {
    let new_coin = Coins::Quarter;
    let new_coin_value = get_value_of_coin(new_coin);
    println!("the coin value is {}", new_coin_value);
}
