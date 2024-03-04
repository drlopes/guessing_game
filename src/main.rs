use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        println!("Guess the number! (between 1 and 100) \n");

        let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
        let mut tries: u32 = 0;

        loop {
            println!("Please input your guess:");

            let mut guess: String = String::new();
            match io::stdin().read_line(&mut guess) {
                Ok(_) => (),
                Err(_) => {
                    println!("Failed to read line!");
                    continue;
                }
            }

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(err) => {
                    println!("{err}. Please type a number!");
                    continue;
                }
            };

            println!("You guessed: {guess} \n");
            tries += 1;

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win! It took you {tries} tries. \n");
                    break;
                }
            }
        }

        println!("Do you want to play again? (yes/no)");

        let mut play_again: String = String::new();
        match io::stdin().read_line(&mut play_again) {
            Ok(_) => (),
            Err(_) => {
                println!("Failed to read line!");
                continue;
            }
        }

        let positive_answers: Vec<&str> = vec![
            "yes",
            "y",
            "yep",
            "yeah",
            "sure",
            "k",
            "ok",
            "okay",
            "indeed",
            "of course",
            "affirmative",
            "1",
        ];

        match positive_answers.contains(&play_again.trim().to_lowercase().as_str()) {
            true => continue,
            false => {
                println!("Goodbye!");
                break;
            }
        }
    }
}
