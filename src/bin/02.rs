use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"; // TODO: Add the test input

#[derive(PartialEq)]
enum Direction {
    ASC,
    DESC,
}

fn get_direction(prev: i32, curr: i32) -> Direction {
    if curr - prev > 0 {
        Direction::ASC
    } else {
        Direction::DESC
    }
}

fn validate_level(level: &Vec<i32>) -> bool {
    let direction: Direction = get_direction(level[0], level[1]);
    for (i, digit) in level.into_iter().enumerate() {
        if i == 0 {
            continue;
        }
	let diff: i32 = (digit - level[i -1]).abs();
        if get_direction(level[i -1], *digit) != direction || diff == 0 || diff > 3 {
            return false
        }        
    }
    true
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let lines = reader.lines().flatten();
        let numbers: Vec<Vec<i32>> = lines.map(|line| line.split(" ").map(|digit| digit.parse().unwrap()).collect()).collect();
        let result: i32 = numbers.iter().fold(0, |acc, level| {
            if validate_level(level) {
                acc + 1 
            } else { 
                acc
            }
        });
        Ok(result)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
