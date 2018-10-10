extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    loop {
        println!("Guess a number 🤔");
        let mut guess = String::new();
        let number = rand::thread_rng().gen_range(1, 100);
        io::stdin().read_line(&mut guess)
           .expect("Failed to read guess...");
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number.");
                continue;
            }
        };
        match guess.cmp(&number) {
            Ordering::Less =>
              println!("Your guess was {} but it was {}. You suck, haha.", guess, number),
            Ordering::Equal => {
               println!("Damn you were right, it was {}.", number);
               break;
           },
            Ordering::Greater =>
               println!("Your guess was {} but it was {}. You suck, haha.", guess, number),
        }
    }
}
