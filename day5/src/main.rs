use std::collections::HashMap;
use std::fs;

fn main() {
    let rules =
        fs::read_to_string("/Users/ken/src/aoc24/day5/src/input.txt").expect("file didn't open");
    let orderings =
        fs::read_to_string("/Users/ken/src/aoc24/day5/src/input2.txt").expect("file didn't open");

    let output = part_one(rules, orderings);
    println!("result: {}", &output)
}

fn part_one(rules: String, orderings: String) -> u32 {
    let rs: HashMap<u32, Vec<u32>> = rules.trim().lines().fold(HashMap::new(), |mut acc, line| {
        match line.split_once("|") {
            Some((x, y)) => {
                let (h, l) = (
                    x.trim().parse::<u32>().unwrap(),
                    y.trim().parse::<u32>().unwrap(),
                );
                acc.entry(h).and_modify(|v| v.push(l)).or_insert(vec![l]);
            }
            _ => (),
        }
        acc
    });

    let ords = orderings
        .trim()
        .lines()
        .map(|line| {
            line.split(",")
                .into_iter()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    check_rules(rs, ords)
}

// checks the position of elements against the rules and failing the ones that break the rules
fn check_rules(rs: HashMap<u32, Vec<u32>>, manual: Vec<Vec<u32>>) -> u32 {
    let mut ans = 0;
    for pages in manual {
        let mut checker = true;
        for (k, v) in rs.iter() {
            for vv in v {
                if pages.contains(&vv)
                    && pages.contains(&k)
                    && pages.iter().position(|x| x == k).unwrap()
                        > pages.iter().position(|x| x == vv).unwrap()
                {
                    checker = false;
                    break;
                }
            }
        }
        if checker {
            ans += pages[pages.len() / 2];
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_one() {}
}
