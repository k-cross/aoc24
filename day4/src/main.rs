use std::collections::HashMap;
use std::fs;

fn main() {
    let content =
        fs::read_to_string("/Users/ken/src/aoc24/day4/src/input.txt").expect("file didn't open");

    let result = perform_calc(content);
    println!("result: {}", result);
}

fn perform_calc(input: String) -> i32 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let (m, n): (usize, usize) = (grid.len(), grid[0].len());
    let mut count = 0;

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 'X' {
                count += check_diagonal(&grid, &i, &j, &m, &n)
                    + check_horizontal(&grid, &i, &j, &m, &n)
                    + check_vertical(&grid, &i, &j, &m, &n);
            }
        }
    }

    count
}

fn check_diagonal(
    grid: &Vec<Vec<char>>,
    row: &usize,
    col: &usize,
    row_size: &usize,
    col_size: &usize,
) -> i32 {
    let POS_ROW = *row + 3 < *row_size;
    let NEG_ROW = *row as i32 - 3 >= 0;
    let POS_COL = *col + 3 < *col_size;
    let NEG_COL = *col as i32 - 3 >= 0;
    let char_map = HashMap::from([(1, 'M'), (2, 'A'), (3, 'S')]);

    let mut pos_pos = true;
    let mut pos_neg = true;
    let mut neg_pos = true;
    let mut neg_neg = true;

    for i in 1..4 {
        if !(POS_ROW && POS_COL && grid[row + i][col + i] == char_map[&i]) {
            pos_pos = false;
        }

        if !(POS_ROW && NEG_COL && grid[row + i][col - i] == char_map[&i]) {
            pos_neg = false;
        }

        if !(NEG_ROW && POS_COL && grid[row - i][col + i] == char_map[&i]) {
            neg_pos = false;
        }

        if !(NEG_ROW && NEG_COL && grid[row - i][col - i] == char_map[&i]) {
            neg_neg = false;
        }
    }

    [pos_pos, neg_neg, neg_pos, pos_neg]
        .iter()
        .fold(0, |mut acc, x| {
            if *x {
                acc += 1;
            }
            acc
        })
}

fn check_horizontal(
    grid: &Vec<Vec<char>>,
    row: &usize,
    col: &usize,
    _row_size: &usize,
    col_size: &usize,
) -> i32 {
    let POS_COL = *col + 3 < *col_size;
    let NEG_COL = *col as i32 - 3 >= 0;
    let char_map = HashMap::from([(1, 'M'), (2, 'A'), (3, 'S')]);
    let mut pos = true;
    let mut neg = true;

    for i in 1..4 {
        if !(POS_COL && grid[*row][col + i] == char_map[&i]) {
            pos = false;
        }

        if !(NEG_COL && grid[*row][col - i] == char_map[&i]) {
            neg = false;
        }
    }

    [pos, neg].iter().fold(0, |mut acc, x| {
        if *x {
            acc += 1;
        }
        acc
    })
}
fn check_vertical(
    grid: &Vec<Vec<char>>,
    row: &usize,
    col: &usize,
    row_size: &usize,
    _col_size: &usize,
) -> i32 {
    let POS_ROW = *row + 3 < *row_size;
    let NEG_ROW = *row as i32 - 3 >= 0;
    let char_map = HashMap::from([(1, 'M'), (2, 'A'), (3, 'S')]);
    let mut pos = true;
    let mut neg = true;

    for i in 1..4 {
        if !(POS_ROW && grid[row + i][*col] == char_map[&i]) {
            pos = false;
        }

        if !(NEG_ROW && grid[row - i][*col] == char_map[&i]) {
            neg = false;
        }
    }

    [pos, neg].iter().fold(0, |mut acc, x| {
        if *x {
            acc += 1;
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use crate::perform_calc;

    #[test]
    fn test_result() {
        //   0 1 2 3 4 5 6 7 8 9
        //0: M M M S X X M A S M
        //1: M S A M X M S M S A
        //2: A M X S X M A A M M
        //3: M S A M A S M S M X
        //4: X M A S A M X A M M
        //5: X X A M M X X A M A
        //6: S M S M S A S X S S
        //7: S A X A M A S A A A
        //8: M A M M M X M M M M
        //9: M X M X A X M A S X
        let s = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"
        .to_string();
        assert_eq!(perform_calc(s), 18);
    }
}
