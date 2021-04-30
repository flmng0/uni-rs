// Ten green bottles project for ICTPRG301 at CDU.
//
// Authored by Timothy Davis on the 30th of April, 2021.
//
// Some brief info about Rust (the programming language
// you're looking at):
//   - Statically typed, with a type inferrence system.
//   - Multi-paradigm.
//   - Compiled, with LLVM used as an IR and compilation
//     backend.
//   - Has packages managed by `Cargo` which pulls from
//     `crates.io`.
//   - Free and open source programming language.

// The entry point of the program.
//
// Note that Rust has order-independent top-level
// declarations.
fn main() {
    println!("{}", sing_bottles(20));
}

// Numbers that don't adhere to the normal format.
//
// This has been commented out because it's hard to use this
// when you're running a Rust program from a single file.
// Normally you would be working in a Cargo project so you
// would be able to add 3rd party dependencies, such as
// "lazy_static", which is used here.
//
// Cargo is Rust's package and project manager.
//
// Unlike Python, Rust doesn't have a global modules system,
// only local to each project.
//
// It does however have a global cache for Cargo to utilize
// incremental compilation.
//
// lazy_static::lazy_static! {
//     static ref SPECIAL_NUMS: HashMap<usize, &'static str>
// = {
//         let mut m = HashMap::new();
//         m.insert(11, "eleven");
//         m.insert(12, "twelve");
//         m.insert(13, "thirteen");
//         m.insert(14, "fourteen");
//         m.insert(15, "fifteen");
//         m.insert(16, "sixteen");
//         m.insert(17, "seventeen");
//         m.insert(18, "eighteen");
//         m.insert(19, "nineteen");
//         m
//     };
// }

// "Multiples of one."
const UNITS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

// Multiples of ten.
const TENS: [&'static str; 9] = [
    "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

// Multiples of one thousand.
const THOUSANDS: [&'static str; 4] = ["thousand", "million", "billion", "trillion"];

// Handle a sub-1000 number.
fn under_1000(n: usize, thousand: usize) -> String {
    assert_ne!(n, 0);

    // This is the same as the SPECIAL_NUMS map from above, it's
    // the same as a dictionary in Python, but slightly more
    // verbose.
    //
    // The disadvantage of defining this map here (rather than
    // above), is that it's initialized on every call to
    // `under_1000`. It may be optimized away by LLVM however.
    let special_nums = {
        use std::collections::HashMap;
        let mut m = HashMap::new();
        m.insert(11, "eleven");
        m.insert(12, "twelve");
        m.insert(13, "thirteen");
        m.insert(14, "fourteen");
        m.insert(15, "fifteen");
        m.insert(16, "sixteen");
        m.insert(17, "seventeen");
        m.insert(18, "eighteen");
        m.insert(19, "nineteen");
        m
    };

    // Get the units and tens place, for special numbers.
    let sub_hundred = n % 100;

    // If the `sub_hundred` number is in the special cases:
    let ten_and_unit = if let Some(word) = special_nums.get(&sub_hundred) {
        // Return it's associated word.
        Some(word.to_string())
    }
    // Otherwise:
    else {
        // Optionally get a string representing the "tens", e.g:
        // "forty".
        let tens = if n >= 10 {
            let tens_place = (n as usize / 10) % 10;
            if tens_place == 0 {
                None
            }
            else {
                Some(TENS[tens_place - 1])
            }
        }
        else {
            None
        };

        // Optionally get a string represting the "units", e.g:
        // "three".
        let units = {
            let units_place = n as usize % 10;
            if units_place == 0 {
                None
            }
            else {
                Some(UNITS[units_place - 1])
            }
        };

        // Get a combined string for the tens and units, e.g:
        // "forty-three".
        //
        // A match statement is like a declaritive switch statement
        // from C/C++. It actually returns the value on the right
        // side of the => arrow.
        //
        // Since this statement is the last statement in this block
        // (and has no ; at the end), it is used as the `return`
        // statement for this block.
        match (tens, units) {
            (Some(ten), Some(unit)) => Some(format!("{}-{}", ten, unit)),
            (Some(ten), None) => Some(ten.to_string()),
            (None, Some(unit)) => Some(unit.to_string()),
            (None, None) => None,
        }
    };

    // Optionally get the number in the hundreds place.
    let hundreds = (n >= 100).then(|| UNITS[(n as usize / 100) - 1]);

    // Combination string of the hundreds place, tens place, and
    // units place.
    let hundred_ten_and_unit = match (hundreds, ten_and_unit) {
        (Some(hundred), Some(ten_and_unit)) => format!("{}-hundred and {}", hundred, ten_and_unit),
        (Some(hundred), None) => format!("{}-hundred", hundred),
        (None, Some(ten_and_unit)) => ten_and_unit,
        (None, None) => unreachable!(),
    };

    // Finally return the resulting string, including the
    // thousands designation.
    if thousand == 0 {
        hundred_ten_and_unit
    }
    else {
        format!("{} {}", hundred_ten_and_unit, THOUSANDS[thousand - 1])
    }
}

// Convert an integer to an English string representing that
// number.
fn english_number(mut n: usize) -> String {
    // The `THOUSANDS` array only goes to trillions.
    assert!(n <= 999_999_999_999_999);

    // Initialize a new Vec (list in Python) which will contain
    // all the `terms` of the number.
    let mut terms = Vec::new();

    // Track which thousand we are up to.
    let mut thousand = 0;
    while n >= 1000 {
        // Get the current sub-1000 number.
        let sub = n % 1000;
        n /= 1000;

        if sub != 0 {
            terms.push(under_1000(sub, thousand));
        }

        thousand += 1;
    }

    terms.push(under_1000(n, thousand));
    // Reversed because of the order which we added the "terms".
    terms.reverse();

    // This is exactly like the str.join(...) method from
    // Python, but the list is what has the method associated
    // with it, instead of the string.
    terms.join(", ")
}

// Sing the green bottles song, with a variable amount of
// starting bottles.
fn sing_bottles(n: usize) -> String {
    // Rust's functional style iterators <3
    (1..=n) // Iterate in [1, n]
        .rev() // in reverse.
        .map(|i| { // Each value in [1, n] is `mappped` to its respective verse.
            let next = i - 1;
            let next_word = if next == 0 {
                "no".to_string()
            }
            else {
                english_number(next)
            };

            let current = english_number(i);
            let current_capitilized = {
                let mut chars = current.chars();
                let head = chars.next().unwrap();
                let tail = chars.collect::<String>();

                format!("{}{}", head.to_uppercase(), tail)
            };

            // The format macro, in combination with Rust's raw string literals.
            format!(
                r#"{0} green bottles
Hanging on the wall
{0} green bottles
Hanging on the wall
And if one green bottle
Should accidentally fall
There'll be {1} green bottles
Hanging on the wall"#,
                current_capitilized, next_word
            )
        })
        .collect::<Vec<String>>()
        .join("\n\n")
}
