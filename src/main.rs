// #![allow(unused)]   // -> will let you to have unused variable in your code

use rand::Rng;
use std::cmp::Ordering;
use std::io; // random -> rang

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new(); // store user given data in this mutable variable
        println!("Guess the number!");

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // match's arm
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You winn!");
                break;
            }
        }
    }
}
