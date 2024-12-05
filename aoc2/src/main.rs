use std::fs;

enum Level {
    Unknown,
    Increasing,
    Decreasing,
}

fn main() {
    let content =
        fs::read_to_string("/Users/ken/src/aoc24/aoc2/src/input.csv").expect("file didn't open");
    let mut safe: i32 = 0;

    for line in content.trim().lines() {
        let mut report = Vec::new();
        for num in line.split(" ") {
            report.push(num.trim().parse::<i32>().unwrap());
        }

        if full_calc(report) {
            safe += 1
        }
    }
    println!("safe reports: {}", safe);
}

pub fn full_calc(report: Vec<i32>) -> bool {
    if calculate_safety(&report) {
        return true;
    } else {
        let len = report.len();
        for i in 0..len {
            let new_r = report
                .iter()
                .enumerate()
                .fold(Vec::new(), |mut acc, (index, val)| {
                    if index != i {
                        acc.push(val.clone());
                    }
                    acc
                });
            dbg!(&new_r);
            if calculate_safety(&new_r) {
                return true;
            }
        }
    }
    false
}

pub fn calculate_safety(report: &Vec<i32>) -> bool {
    let mut saftey = Level::Unknown;

    for (index, num) in report[1..].iter().enumerate() {
        match (report[index] - num, &saftey) {
            (val, Level::Unknown) if val > 0 && val < 4 => saftey = Level::Decreasing,
            (val, Level::Unknown) if val < 0 && val > -4 => saftey = Level::Increasing,
            (val, Level::Increasing) if val < 0 && val > -4 => continue,
            (val, Level::Decreasing) if val > 0 && val < 4 => continue,
            _ => return false,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::full_calc;

    #[test]
    fn test_front_report() {
        assert!(full_calc(vec![1, 1, 2, 3, 4, 5]));
        assert!(full_calc(vec![1, 7, 8, 9]));
        assert!(full_calc(vec![5, 4, 8, 9]));
    }

    #[test]
    fn test_mid_report() {
        assert!(full_calc(vec![1, 2, 2, 3, 4, 5]));
        assert!(full_calc(vec![1, 2, 1, 3, 4, 5]));
        assert!(full_calc(vec![1, 2, 5, 3, 4, 5]));
        assert!(full_calc(vec![1, 2, 3, 3, 5]));
    }

    #[test]
    fn test_back_report() {
        assert!(full_calc(vec![1, 2, 3, 4, 5, 5]));
        assert!(full_calc(vec![1, 2, 3, 4, 5, 10]));
    }
}
