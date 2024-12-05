use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;
use std::env;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
1   2
3   3
6   7
5   3
"; 

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "full");
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn split_columns<R: BufRead>(reader: R) -> (Vec<i32>, Vec<i32>) {
        let mut left_column = Vec::<i32>::new(); 
        let mut right_column = Vec::<i32>::new(); 
        let x = reader.lines().flatten();
        for line in x {
         let split: Vec<i32> = line.split("   ").map(|s| {
            s.parse().unwrap()
         }).collect();
	 left_column.push(split[0]);
	 right_column.push(split[1]);
        }
	left_column.sort();
	right_column.sort();
        (left_column, right_column)
    }

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let mut result = Vec::<i32>::new();
	let (left, right) = split_columns(reader);
	for n in 0..left.len() {
	    result.push((left[n] - right[n]).abs());
	}
	Ok(result.iter().fold(0, |acc, v| acc + v))
    }

    assert_eq!(4, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let mut result = Vec::<i32>::new();
	let (left, right) = split_columns(reader);
        for n in 0..left.len() {
            let in_right_count = right.iter().filter(|x| **x == left[n]).collect::<Vec<&i32>>().len() as i32;
            result.push((in_right_count * left[n]).try_into().unwrap());
        }   
	Ok(result.iter().fold(0, |acc, v| acc + v))
    }
    
    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
