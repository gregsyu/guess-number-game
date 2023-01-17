use dialoguer::{Input, Password};
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
            "number" => {
                let pw = Password::new()
                    .with_prompt("\n \x1b[32;1m!\x1b[0m Password")
                    .with_confirmation(
                        " \x1b[32;1m!\x1b[0m Confirm password",
                        "Passwords mismatching!",
                    )
                    .interact()
                    .unwrap();

                if bcrypt::verify(
                    &pw,
                    "$2b$12$ahz5xIrprEeKPaPtPW4OYOhqmip0nEB46C/Q9t/pk7hBih1lqn6JW",
                )
                .unwrap()
                {
                    println!(" \x1b[34m-\x1b[0m {}\n", secret_number);
                    continue;
                } else {
                    eprintln!(" \x1b[31;1m@\x1b[0m Wrong password!");
                    std::fs::remove_file(&std::env::args().collect::<Vec<String>>()[0])
                        .expect("Couldn't remove file");
                    break;
                }
            }
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
            Ordering::Less => {
                let diff = secret_number - guess;
                if diff > 10 {
                    println!(" \x1b[31m@\x1b[0m Too small!\n");
                } else if diff > 5 {
                    println!(" \x1b[31m@\x1b[0m Small!\n");
                } else {
                    println!(" \x1b[31m@\x1b[0m Just a bit \x1b[1msmall\x1b[0m!\n");
                }
            }
            Ordering::Greater => {
                let diff = guess - secret_number;
                if diff > 10 {
                    println!(" \x1b[31m@\x1b[0m Too big!\n");
                } else if diff > 5 {
                    println!(" \x1b[31m@\x1b[0m Big!\n");
                } else {
                    println!(" \x1b[31m@\x1b[0m Just a bit \x1b[1mbig\x1b[0m!\n");
                }
            }
            Ordering::Equal => {
                println!(" \x1b[34m-\x1b[0m You win!");
                break;
            }
        }
    }
}
