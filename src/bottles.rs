use std::collections::HashMap;

fn main() {
    println!("{}", sing_bottles(20));
}

// Numbers that don't adhere to the normal format.
lazy_static::lazy_static! {
    static ref SPECIAL_NUMS: HashMap<usize, &'static str> = {
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
}

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

    let sub_hundred = n % 100;

    let ten_and_unit = if let Some(word) = SPECIAL_NUMS.get(&sub_hundred) {
        Some(word.to_string())
    }
    else {
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

        let units = {
            let units_place = n as usize % 10;
            if units_place == 0 {
                None
            }
            else {
                Some(UNITS[units_place - 1])
            }
        };

        match (tens, units) {
            (Some(ten), Some(unit)) => Some(format!("{}-{}", ten, unit)),
            (Some(ten), None) => Some(ten.to_string()),
            (None, Some(unit)) => Some(unit.to_string()),
            (None, None) => None,
        }
    };

    let hundreds = (n >= 100).then(|| UNITS[(n as usize / 100) - 1]);

    let hundred_ten_and_unit = match (hundreds, ten_and_unit) {
        (Some(hundred), Some(ten_and_unit)) => format!("{}-hundred and {}", hundred, ten_and_unit),
        (Some(hundred), None) => format!("{}-hundred", hundred),
        (None, Some(ten_and_unit)) => ten_and_unit,
        (None, None) => unreachable!(),
    };

    if thousand == 0 {
        hundred_ten_and_unit
    }
    else {
        format!("{} {}", hundred_ten_and_unit, THOUSANDS[thousand - 1])
    }
}

fn english_number(mut n: usize) -> String {
    assert!(n <= 999_999_999_999_999);

    let mut terms = Vec::new();

    let mut thousand = 0;
    while n >= 1000 {
        let sub = n % 1000;
        n /= 1000;

        if sub != 0 {
            terms.push(under_1000(sub, thousand));
        }

        thousand += 1;
    }

    terms.push(under_1000(n, thousand));
    //
    terms.reverse();

    terms.join(", ")
}

fn sing_bottles(n: usize) -> String {
    (1..=n)
        .rev()
        .map(|i| {
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
