use std::collections::HashSet;
use std::fs;

fn main() {
    let world =
        fs::read_to_string("/Users/ken/src/aoc24/day6/src/input.txt").expect("file didn't open");

    let output = part_one(world);
    println!("result: {}", &output)
}

fn part_one(world: String) -> u32 {
    let l: Vec<&str> = world.trim().lines().collect();
    let row_limit = l.len() as i32;
    let col_limit = l[0].len() as i32;

    let (start, blockers): ((i32, i32), HashSet<(i32, i32)>) = world
        .trim()
        .lines()
        .enumerate()
        .fold(((0, 0), HashSet::new()), |mut acc, (row, line)| {
            acc =
                line.chars()
                    .enumerate()
                    .into_iter()
                    .fold(acc, |(mut start, mut hs), (col, c)| {
                        if c == '#' {
                            hs.insert((row as i32, col as i32));
                        } else if c == '^' {
                            start = (row as i32, col as i32);
                        }

                        (start, hs)
                    });
            acc
        });

    calculate_steps(&row_limit, &col_limit, start, &blockers)
}

fn calculate_steps(
    row_limit: &i32,
    col_limit: &i32,
    mut current: (i32, i32),
    blocks: &HashSet<(i32, i32)>,
) -> u32 {
    // (row, col) which is (y, x) where y+ is down
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut cd = 0;
    let mut seen = HashSet::new();
    while current.0 < *row_limit && current.0 >= 0 && current.1 < *col_limit && current.1 >= 0 {
        if blocks.contains(&(current.0 + dirs[cd].0, current.1 + dirs[cd].1)) {
            cd = (cd + 1) % 4;
        }
        current = (current.0 + dirs[cd].0, current.1 + dirs[cd].1);
        seen.insert(current);
    }

    seen.len() as u32
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_one() {}
}
