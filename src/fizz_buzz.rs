fn fizz_buzz(value: usize) -> String {
    let mut out = (1..=value)
        .map(|i| {
            let mut out = String::new();
            if i % 3 == 0 {
                out.push_str("Fizz");
            }
            if i % 5 == 0 {
                out.push_str("Buzz");
            }
            if out.is_empty() {
                out = i.to_string();
            }
            out
        })
        .fold(String::new(), |acc, v| format!("{}{},", acc, v.as_str()));

    out.truncate(out.len() - 1);

    out
}

fn main() {
    println!("{}", fizz_buzz(20));
}
