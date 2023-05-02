
//  requirement for the guessing game
//  [x] 1. generate a random number
//  [x] 2. prompt the user to input a number
//  [x] 3. read the input
//  [] 4. compare the input with the random number
//  [] 5. if the input is greater than the random number, print "Too big!"
//  [] 6. if the input is less than the random number, print "Too small!"
//  [] 7. if the input is equal to the random number, print "You win!"

use rand::Rng;

use std::io;
fn main() {
    let random_number =  rand::thread_rng().gen_range(1..101);
    println!(" the random number is: {}", random_number );
    println!("Input the number:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).unwrap();
    println!("you just entered : {}", guess)

}
