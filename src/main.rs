// The prelude is the list of things that Rust automatically imports into
// every Rust program. It's kept as small as possible, and is focused on
// things, particualarly traits, which are used in almost every single
//                         Rust program

// The io library comes from the standard library, known as std
use rand::Rng; // We need to import this struct for use `gen_range` method
use std::cmp::Ordering;
use std::io;

fn main() {
    println!(" ------  Guess the number  ------\n");

    let secret_number = rand::thread_rng().gen_range(1..101); // `gen_range(1..=100)` both are equivalent. strange syntax i know.

    println!(" -NOTE-- Enter `quit` to exit ---");

    loop {
        println!(" ------  Input your guess  ------");

        // The `::` syntax in the `::new` line indicates that new is an associated function of the `String` type.
        // An associated function is a function that’s implemented on a type, in this case String. This new function
        // creates a new, empty string. You’ll find a new function on many types, because it’s a common name
        // for a function that makes a new value of some kind.

        let mut guess: String = String::new();

        // Now we’ll call the stdin function from the io module, which will allow us to handle user input:
        io::stdin()
            .read_line(&mut guess)
            // Rust will warns if you remove `expect` function cuz you'll haven’t used the Result value returned from `read_line`, indicating that the program hasn’t handled a possible error.
            .expect("Failed to read line");

        // `trim()` will eliminate any whitespace at the beggining and end. also it will remove `\n` and `\r`.
        guess = guess.trim().to_string();

        match guess.to_lowercase().as_str() {
            "quit" | "exit" => break,
            _ => {}
        }

        // let guess: u32 = guess.trim().parse().expect("Please type a positive number!");

        // Shadowing lets us reuse the `guess` variable name rather than forcing us to create two unique variables, such as `guess_str` and `guess` for example.
        // We use shadowing when we want to convert a data type for other data type, and reuse the same variable name.
        // The `guess` in the expression below refers to the original `guess` variable. the value of original `guess` will be reused on the new `guess` variable.
        // `parse()` strings can be converted to integers in Rust through it.
        let guess: i32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!(" ---  Please, Enter a number  ---\n");
                continue;
            }
        };

        // Pattern Match:
        match guess.cmp(&secret_number) {
            // The Ordering type is another enum and has the variants Less, Greater, and Equal.
            // These are the three outcomes that are possible when you compare two values.
            Ordering::Less => println!(" ---------- Too small! ----------\n"),
            Ordering::Greater => println!(" ----------- Too big! -----------\n"),
            Ordering::Equal => {
                println!(" ++++++++++ You win! ++++++++++"); // Adding `break` line after `You win!` makes the program exit the loop when the user guesses the secret number correctly.
                break; // Exiting the loop also means exiting the program, because the loop is the last part of `main`. (`main` is the point where the program starts).
            }
        }
    }
}
