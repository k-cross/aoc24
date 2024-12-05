use regex::Regex;
use std::fs;

fn main() {
    let content =
        fs::read_to_string("/Users/ken/src/aoc24/aoc4/src/input.txt").expect("file didn't open");
    let mut v: Vec<&str> = Vec::new();

    for line in content.lines() {
        line.
    }

    dbg!(&v);
    let result = perform_calc(v);
    println!("result: {}", result);
}

fn perform_calc(split_input: Vec<&str>) -> i32 {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_front_report() {}
}
