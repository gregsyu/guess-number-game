use dialoguer::Input;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

fn main() {
    println!(" \x1b[34m-\x1b[0m Guess the number\n");
    let secret_number = thread_rng().gen_range(1..101);

    loop {
        let guess: String = Input::new()
            .with_prompt(" \x1b[32m!\x1b[0m Input your guess")
            .interact_text()
            .unwrap();

        match guess.as_str() {
            "quit" | "exit" => break,
            _ => {}
        }

        let guess: isize = match guess.parse() {
            Ok(num) => {
                if num < 1 {
                    println!(" \x1b[31;1m@\x1b[0m Please, Enter a number greater than 0\n");
                    continue;
                } else if num > 100 {
                    println!(" \x1b[31;1m@\x1b[0m Please, Enter a number less than 100\n");
                    continue;
                } else {
                    num
                }
            }
            Err(_) => {
                println!(" \x1b[31;1m@\x1b[0m Please, Enter a valid number\n");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!(" \x1b[31m@\x1b[0m Too small!\n"),
            Ordering::Greater => println!(" \x1b[31m@\x1b[0m Too big!\n"),
            Ordering::Equal => {
                println!(" \x1b[34m-\x1b[0m You win!");
                break;
            }
        }
    }
}
