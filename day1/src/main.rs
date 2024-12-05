use std::collections::HashMap;
use std::fs;

fn main() {
    let content =
        fs::read_to_string("/Users/ken/src/aoc24/day1/src/input.csv").expect("file didn't open");
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut counts = HashMap::new();

    for line in content.trim().lines() {
        let (l_num, r_num) = line.split_once(" ").unwrap();
        left.push(l_num.trim().parse::<u32>().unwrap());
        right.push(r_num.trim().parse::<u32>().unwrap());
        counts
            .entry(r_num.trim().parse::<u32>().unwrap())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    left.sort();
    right.sort();

    println!(
        "distance: {}\nsimilarity: {}",
        calculate_distances(&left, &right),
        calculate_similarity(&counts, &left)
    );
}

pub fn calculate_similarity(counts: &HashMap<u32, u32>, l: &Vec<u32>) -> u32 {
    l.iter().fold(0, |mut acc, num| {
        acc += num * counts.get(&num).or_else(|| Some(&0)).unwrap();
        acc
    })
}

pub fn calculate_distances(left: &Vec<u32>, right: &Vec<u32>) -> u32 {
    left.iter().enumerate().fold(0, |mut acc, (index, num)| {
        acc += num.abs_diff(right[index]);
        acc
    })
}

#[cfg(test)]
mod tests {
    use crate::calculate_distances;

    #[test]
    fn test_distance_sum() {
        let left: Vec<u32> = vec![1, 2, 3];
        let right: Vec<u32> = vec![3, 2, 1];

        assert_eq!(calculate_distances(&left, &right), 4)
    }
}
