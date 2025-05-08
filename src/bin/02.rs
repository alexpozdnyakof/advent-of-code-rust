use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::env;
use std::fs::File;
use std::io::Lines;
use std::io::{BufRead, BufReader};
use std::iter::Flatten;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn validate_level(level: &Vec<i32>) -> bool {
    for (i, _) in level.into_iter().enumerate() {
        let prev = if i == 0 {
            if level[i] - level[i + 1] >= 0 {
                level[i] + 1
            } else {
                level[i] - 1
            }
        } else {
            level[i - 1]
        };
        let next = if i == level.len() - 1 {
            if level[i] - level[i - 1] >= 0 {
                level[i] + 1
            } else {
                level[i] - 1
            }
        } else {
            level[i + 1]
        };
        let diff_prev: i32 = level[i] - prev;
        let diff_next: i32 = level[i] - next;
        if !(((diff_prev > 0 && diff_next < 0) || (diff_prev < 0 && diff_next > 0))
            && diff_prev.abs() != 0
            && diff_prev.abs() <= 3
            && diff_next.abs() != 0
            && diff_next.abs() <= 3)
        {
            return false;
        }
    }
    true
}

fn try_revalidate_level(level: &Vec<i32>) -> bool {
    if validate_level(level) {
        return true;
    };
    for (i, _curr) in level.into_iter().enumerate() {
        let mut cloned = level.clone();
        cloned.remove(i);
        if validate_level(&cloned) {
            return true;
        };
    }
    false
}

fn get_levels_from_raw<R: BufRead>(lines: Flatten<Lines<R>>) -> Vec<Vec<i32>> {
    lines
        .map(|line| {
            line.split(" ")
                .map(|digit| digit.parse().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i32>>>()
}

fn main() -> Result<()> {
    start_day(DAY);
    env::set_var("RUST_BACKTRACE", "full");
    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let numbers = get_levels_from_raw(reader.lines().flatten());
        let result: i32 = numbers.iter().fold(
            0,
            |acc, level| {
                if validate_level(level) {
                    acc + 1
                } else {
                    acc
                }
            },
        );
        Ok(result)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let mut levels = get_levels_from_raw(reader.lines().flatten());

        let result: i32 = levels.iter_mut().fold(0, |acc, level| {
            if try_revalidate_level(&level) {
                acc + 1
            } else {
                acc
            }
        });

        Ok(result)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
