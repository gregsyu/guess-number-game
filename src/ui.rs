const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn help() {
    banner();
    println!("\nSimplest Guess The Number game ever\n");
    println!("Usage:");
    println!("  guess_number [OPTIONS]\n");
    println!("Options:");
    println!("  --help       Show this help message and exit");
    println!("  --version    Show the version information\n");
    println!("Commands:");
    println!("  quit            Exit the game");
    println!("  save            Save current progress");
    println!("  results         View game results");
    println!("  name            Change player name");
    println!("  restart         Start a new game");
    println!("  number          Reveal the secret number (requires password)\n");
    println!("Examples:");
    println!("  guess_number             Start a new game");
    println!("  guess_number results     Show game results");
}

pub fn banner() {
    println!("Guess the number \x1b[1m({})\x1b[0m", VERSION);
}

pub fn dash_banner() {
    println!(
        " \x1b[38;5;250m-\x1b[0m Guess the number \x1b[1m({})\x1b[0m",
        VERSION
    );
}

pub fn goodbye() {
    println!(" \x1b[34;1m-\x1b[0m Thanks for playing. Goodbye!");
    std::thread::sleep(std::time::Duration::from_millis(500)); // sleeps 0.5 seconds
}

pub fn results<T>(name: &String, total_guesses: T, guesses: T, total_tries: T, tries: T) -> ()
where
    T: std::fmt::Display,
{
    println!(" \x1b[34m-\x1b[0m Results from the Player: \x1b[1m{name}\x1b[0m");
    println!(" \x1b[34m-\x1b[0m Number of Attempts this Round: \x1b[1m{tries}\x1b[0m");
    println!(" \x1b[34m-\x1b[0m Number of Total Attempts: \x1b[1m{total_tries}\x1b[0m");
    println!(" \x1b[34m-\x1b[0m Number of Guesses in This Process: \x1b[1m{guesses}\x1b[0m");
    println!(
        " \x1b[34m-\x1b[0m Number of Total Guesses in all Games: \x1b[1m{total_guesses}\x1b[0m"
    );
}
