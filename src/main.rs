use dialoguer::{Input, Password};
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::path::Path;

const PW_PATH: &str = "pw.txt";
const CSV_FILE_PATH: &str = "results.csv";

fn main() -> anyhow::Result<()> {
    println!(" \x1b[38;5;250m-\x1b[0m Guess the number \x1b[1m(0.1.4)\x1b[0m");
    // Takes the values of `guesses`, `total_tries` and `name` from `CSV_FILE_PATH`
    let (mut name, mut guesses, mut total_tries) = if Path::new(CSV_FILE_PATH).exists() {
        let file = File::open(CSV_FILE_PATH)?;
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);

        // `find_map` iterates until it finds the first `Some` variant
        let result = rdr.records().find_map(|result| {
            let record = result.unwrap();
            if let (Some(name), Some(total_tries_str), Some(guesses_str)) = (record.get(0), record.get(2), record.get(1)) {
                Some((
                    String::from(name),
                    guesses_str.parse().unwrap_or(0),
                    total_tries_str.parse().unwrap_or(0),
                ))
            } else {
                None
            }
        });

        result.unwrap_or((user("unknown"), 0, 0))
    } else {
        (user("player"), 0, 0)
    };

    'game: loop {
        let secret_number = thread_rng().gen_range(1..101);
        let mut tries = 1;

        'guess: loop {
            let sign: String = if tries == 1 {
                String::from("!")
            } else {
                tries.to_string()
            };
            let guess: String = Input::new()
                .with_prompt(format!("\n \x1b[32m{}\x1b[0m Input your guess", sign))
                .interact_text()?;

            match guess.as_str() {
                "quit" | "exit" => {
                    goodbye(None);
                    break 'game;
                }
                "export" => {
                    Exporter::new()
                        .create(true)
                        .print(true)
                        .file(CSV_FILE_PATH)
                        .export(guesses, total_tries, &name)?;
                    continue;
                }
                "results" => {
                    print_results(&name, guesses, total_tries, tries);
                    continue;
                }
                "name" => {
                    let new_name: String = Input::new()
                        .with_prompt("\n \x1b[32;1m·\x1b[m Name")
                        .interact_text()?;
                    name = new_name.trim().replace(" ", "_");
                    continue;
                }
                "restart" => {
                    println!("\n \x1b[34m-\x1b[0m New game");
                    break 'guess;
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
                        println!(" \x1b[34m-\x1b[0m {}", secret_number);
                        continue;
                    } else {
                        eprintln!(" \x1b[31;1m@\x1b[0m Wrong password!");
                        match std::fs::remove_file(&std::env::args().collect::<Vec<String>>()[0]) {
                            Ok(_) => eprintln!(" \x1b[31;1m@\x1b[0m Where's your file? >\x1b[31;1m:\x1b[0m^"),
                            Err(_) => eprintln!(" \x1b[31;1m@\x1b[0m Next time i'll remove your file! >\x1b[31;1m:\x1b[0m("),
                        }
                        std::process::exit(255);
                    }
                }
                _ => {}
            }

            let guess: isize = match guess.parse() {
                Ok(num) => {
                    if num < 1 {
                        println!(" \x1b[31;1m@\x1b[0m Please, Enter a number greater than 0");
                        continue;
                    } else if num > 100 {
                        println!(" \x1b[31;1m@\x1b[0m Please, Enter a number less than 100");
                        continue;
                    } else {
                        num
                    }
                }
                Err(_) => {
                    println!(" \x1b[31;1m@\x1b[0m Please, Enter a valid number");
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
                    guesses += 1;
                    total_tries += tries;

                    println!(" \x1b[34;1m-\x1b[0m You win!\n");
                    print_results(&name, guesses, total_tries, tries);
                    let new_game: String = Input::new()
                        .with_prompt(" \x1b[34m?\x1b[0m New Game? [Y/n/e]")
                        .default("e".to_string())
                        .show_default(false)
                        .interact_text()?;
                    let exporter = Exporter::new().file(CSV_FILE_PATH);

                    match new_game.trim().to_lowercase().as_str() {
                        "y" | "yes" => {
                            // Only exports to the file if it exists else it does nothing
                            exporter.export(guesses, total_tries, &name).unwrap_or(());
                            break 'guess;
                        }
                        "e" | "export" => {
                            exporter
                                .create(true)
                                .print(true)
                                .export(guesses, total_tries, &name)?;
                        }
                        _ => {
                            exporter.export(guesses, total_tries, &name).unwrap_or(());
                            goodbye(Some("\n"));
                        }
                    }
                    break 'game;
                }
            }
        }
    }

    Ok(())
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

    fn export<T>(&self, guesses: T, total_tries: T, name: &String) -> anyhow::Result<()>
    where
        T: ToString,
    {
        let file = OpenOptions::new()
            .create(self.create)
            .write(true)
            .truncate(true)  // removes the content of the file before writing
            .open(&self.file_path)?;
        let mut wtr = csv::Writer::from_writer(file);

        wtr.write_record(&["player", "total guesses", "total attempts"])?;
        wtr.write_record(&[name, &guesses.to_string(), &total_tries.to_string()])?;
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

fn print_results<T>(name: &String, guesses: T, total_tries: T, tries: T) -> ()
where
    T: std::fmt::Display,
{
    println!(" \x1b[34m-\x1b[0m Results from the Player: \x1b[1m{name}\x1b[0m");
    println!(" \x1b[34m-\x1b[0m Number of Attempts this Round: \x1b[1m{tries}\x1b[0m");
    println!(" \x1b[34m-\x1b[0m Number of Total Attempts: \x1b[1m{total_tries}\x1b[0m");
    println!(" \x1b[34m-\x1b[0m Number of Total Guesses: \x1b[1m{guesses}\x1b[0m");
}

fn goodbye(beginning_str: Option<&str>) -> () {
    println!(
        "{} \x1b[34;1m-\x1b[0m Thanks for playing. Goodbye!",
        beginning_str.unwrap_or("")
    );
    std::thread::sleep(std::time::Duration::from_millis(500)); // sleeps 0.5 seconds
}

fn user(name: &str) -> String {
    match std::env::var("USER") {
        Ok(user) => user,
        Err(_) => String::from(name),
    }
}
