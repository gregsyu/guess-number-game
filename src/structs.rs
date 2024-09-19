use csv::Writer;
use rand::{thread_rng, Rng};
use std::fs::OpenOptions;

pub struct GameResults {
    pub secret_number: isize,
    pub tries: usize,
    pub total_guesses: usize,
    pub guesses: usize,
    pub total_tries: usize,
    pub name: String,
}

impl GameResults {
    pub fn new() -> Self {
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

    pub fn update_attempts(&mut self) {
        self.tries += 1;
    }

    pub fn record_guess(&mut self) {
        self.total_guesses += 1;
        self.guesses += 1;
        self.total_tries += self.tries;
    }

    pub fn new_game(&mut self) {
        self.tries = 1;
        self.secret_number = thread_rng().gen_range(1..=100);
    }
}

pub struct Exporter {
    file_path: String,
    create: bool,
    print: bool,
}

impl Exporter {
    pub fn new() -> Self {
        Exporter {
            file_path: String::new(),
            create: false,
            print: false,
        }
    }

    pub fn file(mut self, file_path: &str) -> Self {
        self.file_path = String::from(file_path);
        self
    }

    pub fn create(mut self, create: bool) -> Self {
        self.create = create;
        self
    }

    pub fn print(mut self, print: bool) -> Self {
        self.print = print;
        self
    }

    pub fn export<T>(&self, total_guesses: T, total_tries: T, name: &String) -> csv::Result<()>
    where
        T: ToString,
    {
        let file = OpenOptions::new()
            .create(self.create)
            .write(true)
            .truncate(true) // removes the content of the file before writing
            .open(&self.file_path)?;
        let mut wtr = Writer::from_writer(file);

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
