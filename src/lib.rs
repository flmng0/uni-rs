use std::io::{self, BufRead, Write};

// Python style `input` method.
pub fn input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);

    io::stdout().flush()?;
    io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .map(|x| x.trim_end().to_owned())
}

// Shorthand return type for the main method if it can
// return fail.
pub type MainResult = Result<(), Box<dyn std::error::Error>>;
