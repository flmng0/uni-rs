use uni_rs::*;

const MINIMUM_LENGTH: usize = 8;

#[derive(Clone, Copy)]
enum Criteria {
    LowercaseChar,
    UppercaseChar,
    Digit,
}

impl Criteria {
    fn describe(self) -> &'static str {
        match self {
            Self::LowercaseChar => "1 lowercase character",
            Self::UppercaseChar => "1 uppercase character",
            Self::Digit => "1 digit",
        }
    }
}

enum ValidationState {
    TooShort,
    Failed(Vec<Criteria>),
    Passed,
}

fn validate(password: String) -> ValidationState {
    if password.len() < MINIMUM_LENGTH {
        return ValidationState::TooShort;
    }

    let mut has_lowercase = false;
    let mut has_uppercase = false;
    let mut has_digit = false;

    for char in password.chars() {
        if char.is_lowercase() {
            has_lowercase = true;
        }

        if char.is_uppercase() {
            has_uppercase = true;
        }

        if char.is_digit(10) {
            has_digit = true;
        }
    }

    let mut failures = Vec::new();

    if !has_lowercase {
        failures.push(Criteria::LowercaseChar);
    }
    if !has_uppercase {
        failures.push(Criteria::UppercaseChar);
    }
    if !has_digit {
        failures.push(Criteria::Digit);
    }

    if !failures.is_empty() {
        ValidationState::Failed(failures)
    }
    else {
        ValidationState::Passed
    }
}

fn main() -> MainResult {
    'input: loop {
        let password = input("Input a password: ")?;
        match validate(password) {
            ValidationState::TooShort => {
                println!("Password must contain a minimum of 8 characters.");
            },
            ValidationState::Failed(failures) => {
                println!(
                    "Your password is invalid. Missing {}. TRY AGAIN",
                    failures
                        .iter()
                        .map(|c| c.describe())
                        .collect::<Vec<&str>>()
                        .join(", ")
                )
            },
            ValidationState::Passed => {
                println!("Password Compliant");
                break 'input;
            },
        }
    }

    Ok(())
}
