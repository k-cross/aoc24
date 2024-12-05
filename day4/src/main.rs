use std::fs;

fn main() {
    let content =
        fs::read_to_string("/Users/ken/src/aoc24/day4/src/input.txt").expect("file didn't open");
    let grid = content
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    dbg!(&grid);
}

fn perform_calc(split_input: Vec<&str>) -> i32 {
    0
}

fn check_diagonal(grid: &Vec<&str>) -> bool {
    false
}
fn check_horizontal(grid: &Vec<&str>) -> bool {
    false
}
fn check_vertical(grid: &Vec<&str>) -> bool {
    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_front_report() {}
}
