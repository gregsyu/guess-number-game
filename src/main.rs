use dialoguer::Input;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

fn main() {
    println!(" - Guess the number\n");
    let secret_number = thread_rng().gen_range(1..101);

    loop {
        let guess: String = Input::new()
            .with_prompt(" ! Input your guess")
            .interact_text()
            .unwrap();

        match guess.as_str() {
            "quit" | "exit" => break,
            _ => {}
        }

        let guess: isize = match guess.parse() {
            Ok(num) => {
                if num < 1 {
                    println!(" @ Please, Enter a number greater than 0\n");
                    continue;
                } else if num > 100 {
                    println!(" @ Please, Enter a number less than 100\n");
                    continue;
                } else {
                    num
                }
            }
            Err(_) => {
                println!(" @ Please, Enter a number\n");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!(" @ Too small!\n"),
            Ordering::Greater => println!(" @ Too big!\n"),
            Ordering::Equal => {
                println!(" - You win!");
                break;
            }
        }
    }
}
