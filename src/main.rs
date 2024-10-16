use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the guessing game!");
    println!("I've selected a number between 1 and 100. Would you like to see if you can guess it in this game of ours?");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut times_guessed = 0;

    loop {
        println!("Please enter your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        times_guessed += 1;

        println!(
            "You guessed: {}, and you have guessed {} time(s). You may guess {} more time(s)",
            guess,
            times_guessed,
            10 - times_guessed
        );

        if times_guessed <= 10 {
            match guess.cmp(&secret_number) {
                Ordering::Less => {
                    println!("{}", difference_message(secret_number - guess, "small"))
                }
                Ordering::Greater => {
                    println!("{}", difference_message(guess - secret_number, "large"))
                }
                Ordering::Equal => {
                    println!("That's it! I bestow upon you the title MASTER OF NUMBERS, it comes with no benefits and doesn't really mean anything besides what you may find it would be fun to mean, if you entertain to consider something that would be damaging to me or anyone else, the title is now revoked! What a shame");
                    break;
                }
            }
        } else {
            println!("Oh no! You're out of guesses! Better luck next time!");
            break;
        }
    }
}

fn difference_message(separation: u32, direction: &str) -> String {
    match separation.cmp(&10) {
        Ordering::Less => format!(
            "Nope, too {}... but you are closer than you think!",
            direction
        ),
        Ordering::Greater => format!("Nope, too {}", direction),
        Ordering::Equal => "SUSPENSE ENSUES! by 10 it is... however, which way?".to_string(),
    }
}
