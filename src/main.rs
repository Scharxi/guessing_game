use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;
use std::result::Result;

enum Diffeculty {
    Easy,
    Medium,
    Hard,
}

fn print_info() {
    println!(
        "Choose your diffeculty: \n
    1. Easy\n
    2. Medium\n
    3. Hard"
    );
}

fn check_diffeculty(input: String) -> Result<Diffeculty, String> {
    match input.to_lowercase().as_str() {
        "1" => Ok(Diffeculty::Easy),
        "2" => Ok(Diffeculty::Medium),
        "3" => Ok(Diffeculty::Hard),
        _ => Err(input),
    }
}

fn set_tries(diffeculty: Diffeculty, tries: &mut u32) {
    match diffeculty {
        Diffeculty::Easy => *tries = 15u32,
        Diffeculty::Medium => *tries = 10u32,
        Diffeculty::Hard => *tries = 5u32,
    };
}

fn main() -> Result<(), ()> {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut tries: u32 = 0;
    let mut max_tries = 10;

    print_info();

    loop {
        // diffeculty
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");

        // let input: u32 = input.trim().parse().expect("This is not a number!");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("This is not a number!");
                continue;
            }
        };

        let diffeculty: Diffeculty = match check_diffeculty(input.to_string()) {
            Ok(diff) => diff, 
            Err(input) => {
                println!("{} is not a valid option\nPlease choose 1,2 or 3", input);
                continue;
            }
        };

        set_tries(diffeculty, &mut max_tries);
        break;
    }

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Tries: {}", tries);

        tries += 1;

        if tries >= max_tries {
            println!("You lost! Out off tries :(");
            break;
        }

        println!("You guessed: {}", guess);
        println!("Tries left: {}", max_tries - tries);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! You made it with {} tries", tries);
                break;
            }
        }
    }
    Ok(())
}
