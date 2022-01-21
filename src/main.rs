use std::io::stdin;
use std::cmp::Ordering;
use rand::{Rng};
use std::result::Result;

enum Diffeculty {
    Easy, Medium, Hard
}

fn print_info() {
    println!("Choose your diffeculty: \n
    1. Easy\n
    2. Medium\n
    3. Hard");
}

fn check_diffeculty(input: String) -> Result<Diffeculty, ()> {
    match input.to_lowercase().as_str() {
        "1" => Ok(Diffeculty::Easy),
        "2" => Ok(Diffeculty::Medium),
        "3" => Ok(Diffeculty::Hard),
        _ => Err(())
    }
}

fn set_tries(diffeculty: Diffeculty, tries: &mut u32) {
    match diffeculty {
        Diffeculty::Easy => *tries = 15u32,
        Diffeculty::Medium => *tries = 10u32,
        Diffeculty::Hard => *tries = 5u32
    };
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut tries: u32 = 0;
    const MAX_TRIES : u32= 10;

    print_info();

    

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        set_tries(check_diffeculty(guess.to_string()).expect("Not a valid diffeculty"), &mut tries);

        println!("Tries: {}", tries);

        tries+=1;

        if tries >= MAX_TRIES {
            println!("You lost! Out off tries :(");
            break;
        }

        println!("You guessed: {}", guess);
        println!("Tries left: {}", MAX_TRIES - tries);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! You made it with {} tries", tries);
                break;
            }
        }
    }
}
