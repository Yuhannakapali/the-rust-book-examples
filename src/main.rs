// fn main() {
//     println!("Hello, World!")
// }

//  requirement for the guessing game
//  [x] 1. generate a random number
//  [x] 2. prompt the user to input a number
//  [x] 3. read the input
//  [] 4. compare the input with the random number
//  [] 5. if the input is greater than the random number, print "Too big!"
//  [] 6. if the input is less than the random number, print "Too small!"
//  [] 7. if the input is equal to the random number, print "You win!"

use rand::Rng;
use std::cmp;
use std::io;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..101);
    loop {
        println!(" the random number is: {}", random_number);
        println!("Input the number:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read the data");

        let quit_string = String::from(":q");

        if guess.trim() == quit_string {
            println!("You quit the game");
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&random_number) {
            cmp::Ordering::Equal => {
                println!("You win");
                break;
            }
            cmp::Ordering::Greater => println!("Greater"),
            cmp::Ordering::Less => {
                println!("less")
            }
        };
    }
}
