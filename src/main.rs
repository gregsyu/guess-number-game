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
            .expect("failed");

        match guess.as_str() {
            "quit" | "exit" => break,
            _ => {}
        }

        let guess: usize = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!(" @ Please, Enter a integer number\n");
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
