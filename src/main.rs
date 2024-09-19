mod ui;
use anyhow::Result;
use dialoguer::{Input, Password};
use once_cell::sync::Lazy;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::env::{args, current_exe, var};
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::path::Path;
use ui::*;

const PW_PATH: &str = "pw.txt";
const CSV_FILE_PATH: &str = "results.csv";
static USER: Lazy<String> = Lazy::new(|| var("USER").unwrap_or(String::from("player")));

fn main() -> Result<()> {
    let mut game = GameResults::new();
    let args = args().collect::<Vec<String>>();
    // Takes the values of `total_guesses`, `total_tries` and `name` from `CSV_FILE_PATH`
    (game.name, game.total_guesses, game.total_tries) = if Path::new(CSV_FILE_PATH).exists() {
        let file = File::open(CSV_FILE_PATH)?;
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);

        // `find_map` iterates until it finds the first `Some` variant
        let result = rdr.records().find_map(|result| {
            let record = result.unwrap();
            if let (Some(name), Some(total_tries_str), Some(total_guesses_str)) =
                (record.get(0), record.get(2), record.get(1))
            {
                Some((
                    String::from(name),
                    total_guesses_str.parse().unwrap_or(0),
                    total_tries_str.parse().unwrap_or(0),
                ))
            } else {
                None
            }
        });

        result.unwrap_or((USER.clone(), 0, 0))
    } else {
        (USER.clone(), 0, 0)
    };

    if args.len() > 1 {
        match args[1].as_str() {
            "-h" | "--help" => help(),
            "-v" | "--version" => banner(),
            "results" => {
                dash_banner();
                results(&game.name, game.total_guesses, 0, game.total_tries, 0);
            }
            _ => eprintln!(
                "unknown option '{}'\nTry `{} -h' for more information.",
                args[1], args[0]
            ),
        }
        return Ok(());
    }

    dash_banner();
    game.new_game();

    loop {
        let sign: String = if game.tries == 1 {
            String::from("!")
        } else {
            game.tries.to_string()
        };
        let guess: String = Input::new()
            .with_prompt(format!("\n \x1b[32m{}\x1b[0m Input your guess", sign))
            .interact_text()?;

        match guess.as_str() {
            "quit" | "exit" => {
                goodbye();
                break;
            }
            guess if guess.contains("save") => {
                Exporter::new()
                    .create(true)
                    .print(true)
                    .file(CSV_FILE_PATH)
                    .export(game.total_guesses, game.total_tries, &game.name)?;
                if guess.contains("quit") {
                    goodbye();
                    break;
                }
                continue;
            }
            "results" => {
                results(
                    &game.name,
                    game.total_guesses,
                    game.guesses,
                    game.total_tries,
                    game.tries,
                );
                continue;
            }
            "name" => {
                println!();
                let new_name: String = Input::new()
                    .with_prompt(" \x1b[32;1m·\x1b[m Name")
                    .validate_with(|input: &String| -> Result<(), &str> {
                        let special = "(){}[]'\"";
                        if !special.chars().any(|c| input.contains(c)) {
                            Ok(())
                        } else {
                            Err("Invalid character.")
                        }
                    })
                    .default(USER.clone().into())
                    .show_default(false)
                    .interact_text()?
                    .trim()
                    .to_string();

                game.name = match new_name.as_str() {
                    "quit" | "exit" | "cancel" => continue,
                    name if name.contains(" ") => {
                        println!(" \x1b[38;5;250m·\x1b[m Changing \x1b[2mspaces\x1b[0m to \x1b[2munderscores\x1b[0m");
                        new_name.replace(" ", "_")
                    }
                    _ => new_name,
                };

                println!(" \x1b[38;5;250;1m·\x1b[m Your new name: \x1b[1m{}\x1b[0m", game.name);
                continue;
            }
            "restart" => {
                println!("\n \x1b[34m-\x1b[0m New game");
                game.new_game();
                continue;
            }
            "number" => {
                // verify if a given file exists
                let pw: String = if Path::new(PW_PATH).exists() {
                    let mut file = File::open(&PW_PATH)?;
                    let mut pw = String::new();
                    file.read_to_string(&mut pw)?;
                    pw
                } else {
                    let pw = Password::new()
                        .with_prompt("\n \x1b[32;1m!\x1b[0m Password")
                        .interact()?;
                    pw
                };

                if bcrypt::verify(
                    &pw.trim(),
                    "$2b$12$ahz5xIrprEeKPaPtPW4OYOhqmip0nEB46C/Q9t/pk7hBih1lqn6JW",
                )? {
                    println!(" \x1b[34m-\x1b[0m {}", game.secret_number);
                    continue;
                } else {
                    eprintln!(" \x1b[31;1m@\x1b[0m Wrong password!");
                    match std::fs::remove_file(current_exe()?) {
                        Ok(_) => eprintln!(" \x1b[31;1m@\x1b[0m Where's your file? >\x1b[31;1m:\x1b[0m^"),
                        Err(_) => eprintln!(" \x1b[31;1m@\x1b[0m Next time i'll remove your file! >\x1b[31;1m:\x1b[0m("),
                    }
                    std::process::exit(255);
                }
            }
            _ => {}
        }

        let guess: isize = match guess.parse() {
            Ok(num) if (1..=100).contains(&num) => num,
            Ok(num) if num < 1 => {
                println!(" \x1b[31;1m@\x1b[0m Please, Enter a number greater than 0");
                continue;
            }
            Ok(num) if num > 100 => {
                println!(" \x1b[31;1m@\x1b[0m Please, Enter a number less than 100");
                continue;
            }
            Ok(_) => continue, // Fallback case
            Err(_) => {
                println!(" \x1b[31;1m@\x1b[0m Please, Enter a valid number");
                continue;
            }
        };

        let ordering = guess.cmp(&game.secret_number);
        let diff = (game.secret_number - guess).abs();

        let feedback_msg = || match ordering {
            Ordering::Less => match diff {
                d if d > 10 => "Too small!",
                d if d > 5 => "Small!",
                _ => "Just a bit \x1b[1msmall\x1b[0m!",
            },
            Ordering::Greater => match diff {
                d if d > 10 => "Too big!",
                d if d > 5 => "Big!",
                _ => "Just a bit \x1b[1mbig\x1b[0m!",
            },
            Ordering::Equal => "You win!",
        };

        if let Ordering::Equal = ordering {
            game.record_guess();

            println!(" \x1b[34;1m-\x1b[0m {}\n", feedback_msg());
            results(&game.name, game.total_guesses, game.guesses, game.total_tries, game.tries);
            let new_game: String = Input::new()
                .with_prompt(" \x1b[34m?\x1b[0m New Game? [Y/n/e]")
                .default("e".to_string())
                .show_default(false)
                .interact_text()?;
            let exporter = Exporter::new().file(CSV_FILE_PATH);

            match new_game.trim().to_lowercase().as_str() {
                "y" | "yes" => {
                    // Only exports to the file if it exists else it does nothing
                    exporter
                        .export(game.total_guesses, game.total_tries, &game.name)
                        .unwrap_or(());
                    game.new_game();
                    continue;
                }
                "e" | "export" => {
                    exporter.create(true).print(true).export(
                        game.total_guesses,
                        game.total_tries,
                        &game.name,
                    )?;
                }
                _ => {
                    exporter
                        .export(game.total_guesses, game.total_tries, &game.name)
                        .unwrap_or(());
                    println!();
                    goodbye();
                }
            }

            break;
        } else {
            println!(" \x1b[31m@\x1b[0m {}", feedback_msg());
            game.update_attempts();
        }
    }

    Ok(())
}

struct GameResults {
    secret_number: isize,
    tries: usize,
    total_guesses: usize,
    guesses: usize,
    total_tries: usize,
    name: String,
}

impl GameResults {
    fn new() -> Self {
        let secret_number = thread_rng().gen_range(1..=100);
        GameResults {
            secret_number,
            tries: 1,
            total_guesses: 0,
            guesses: 0,
            total_tries: 0,
            name: String::new(),
        }
    }

    fn update_attempts(&mut self) {
        self.tries += 1;
    }

    fn record_guess(&mut self) {
        self.total_guesses += 1;
        self.guesses += 1;
        self.total_tries += self.tries;
    }

    fn new_game(&mut self) {
        self.tries = 1;
        self.secret_number = thread_rng().gen_range(1..=100);
    }
}

struct Exporter {
    file_path: String,
    create: bool,
    print: bool,
}

impl Exporter {
    fn new() -> Self {
        Exporter {
            file_path: String::new(),
            create: false,
            print: false,
        }
    }

    fn file(mut self, file_path: &str) -> Self {
        self.file_path = String::from(file_path);
        self
    }

    fn create(mut self, create: bool) -> Self {
        self.create = create;
        self
    }

    fn print(mut self, print: bool) -> Self {
        self.print = print;
        self
    }

    fn export<T>(&self, total_guesses: T, total_tries: T, name: &String) -> Result<()>
    where
        T: ToString,
    {
        let file = OpenOptions::new()
            .create(self.create)
            .write(true)
            .truncate(true) // removes the content of the file before writing
            .open(&self.file_path)?;
        let mut wtr = csv::Writer::from_writer(file);

        wtr.write_record(&["player", "total guesses", "total attempts"])?;
        wtr.write_record(&[name, &total_guesses.to_string(), &total_tries.to_string()])?;
        wtr.flush()?;

        if self.print {
            println!(
                "\n \x1b[34;1m*\x1b[0m Exporting to file: {}",
                &self.file_path
            );
        }

        Ok(())
    }
}
