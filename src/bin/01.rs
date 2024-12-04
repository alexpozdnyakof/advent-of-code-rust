use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;
use std::env;

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
<TEST-INPUT>
"; // TODO: Add the test input

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "full");
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        //let answer = reader.lines().flatten().count();
        let mut left_column = Vec::<i32>::new(); 
        let mut right_column = Vec::<i32>::new(); 
        let mut result = Vec::<i32>::new(); 
        let x = reader.lines().flatten().enumerate();
        for (i, line) in x {
         let split: Vec<i32> = line.split("   ").map(|s| {
            s.parse().unwrap()
         }).collect();
	 left_column.push(split[0]);
	 right_column.push(split[1]);
         println!("{} {:?}", i, split);
        }
	left_column.sort();
	right_column.sort();
	
	for n in 0..left_column.len() {
	    result.push((left_column[n] - right_column[n]).abs());
	}
	let sum = result.into_iter().reduce(|a, b| a + b); 
        println!("{:?}", left_column);
        println!("{:?}", right_column);
        println!("{:?}", sum);
    	
	Ok(1)
    }

    // TODO: Set the expected answer for the test input
    //assert_eq!(1, part1(BufReader::new(TEST.as_bytes()))?);

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
