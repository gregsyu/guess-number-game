use dialoguer::{Input, Password};
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::fs::{metadata, File};
use std::io::Read;

fn main() {
    println!(" \x1b[34m-\x1b[0m Guess the number \x1b[1m(0.1.3)\x1b[0m");
    let mut count: u16 = 0;
    let mut total_tries: u8 = 0;

    'game: loop {
        let secret_number = thread_rng().gen_range(1..101);
        let mut tries: u8 = 1;

        'guess: loop {
            let sign: String = if tries == 1 {
                String::from("!")
            } else {
                tries.to_string()
            };
            let guess: String = Input::new()
                .with_prompt(format!("\n \x1b[32m{}\x1b[0m Input your guess", sign))
                .interact_text()
                .unwrap();

            match guess.as_str() {
                "quit" | "exit" => break 'game,
                "number" => {
                    let file_path: &str = "pw.txt";
                    let pw: String = if metadata(&file_path).is_ok() { // verify if a given file exists
                        let mut file = File::open(&file_path).unwrap();
                        let mut pw = String::new();
                        file.read_to_string(&mut pw).unwrap();
                        pw
                    } else {
                        let pw = Password::new()
                            .with_prompt("\n \x1b[32;1m!\x1b[0m Password")
                            .interact()
                            .unwrap();
                        pw
                    };

                    if bcrypt::verify(
                        &pw.trim(),
                        "$2b$12$ahz5xIrprEeKPaPtPW4OYOhqmip0nEB46C/Q9t/pk7hBih1lqn6JW",
                    )
                    .unwrap()
                    {
                        println!(" \x1b[34m-\x1b[0m {}", secret_number);
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
                        println!(" \x1b[31m@\x1b[0m Too small!");
                    } else if diff > 5 {
                        println!(" \x1b[31m@\x1b[0m Small!");
                    } else {
                        println!(" \x1b[31m@\x1b[0m Just a bit \x1b[1msmall\x1b[0m!");
                    }
                    tries += 1;
                }
                Ordering::Greater => {
                    let diff = guess - secret_number;
                    if diff > 10 {
                        println!(" \x1b[31m@\x1b[0m Too big!");
                    } else if diff > 5 {
                        println!(" \x1b[31m@\x1b[0m Big!");
                    } else {
                        println!(" \x1b[31m@\x1b[0m Just a bit \x1b[1mbig\x1b[0m!");
                    }
                    tries += 1;
                }
                Ordering::Equal => {
                    count += 1;
                    total_tries += tries;

                    println!(" \x1b[34;1m-\x1b[0m You win!");
                    println!(" \x1b[34m-\x1b[0m Number of Attempts this Round: \x1b[1m{tries}\x1b[0m");
                    println!(" \x1b[34m-\x1b[0m Number of Total Attempts: \x1b[1m{total_tries}\x1b[0m");
                    println!(" \x1b[34m-\x1b[0m Number of Total Guesses: \x1b[1m{count}\x1b[0m");
                    let new_game: String = Input::new()
                        .with_prompt(" \x1b[34m?\x1b[0m New Game? [Y/n]")
                        .interact_text()
                        .unwrap();

                    if new_game.trim().to_lowercase() != "y" {
                        println!("\n \x1b[34;1m-\x1b[0m Thanks for playing. Goodbye!");
                        break 'game;
                    }
                    break 'guess;
                }
            }
        }
    }
}
