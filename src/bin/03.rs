use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead + std::fmt::Debug>(reader: R) -> Result<usize> {
        let answer = reader.lines();
        let mut result = 0;
        for line in answer.into_iter() {
            let chars_vec: Vec<_> = line.unwrap().chars().collect();
            let mut parsing_mode: bool = false;
            let mut cache = String::new();
            let mut n: usize = 0; 
            while n < chars_vec.len() {
                if parsing_mode {
                   if chars_vec[n].is_numeric() || chars_vec[n] == ',' { 
                       cache.push(chars_vec[n]);
                   } else {
                       if chars_vec[n] == ')' { 
                           let multiple_result = cache.split(",").fold(1, |acc, v| {
                               acc * v.parse::<usize>().unwrap()
                           });
                           result += multiple_result;
                       }
                       cache = String::new();
                       parsing_mode = false;
                   }
                }
                if chars_vec[n].is_numeric() && &chars_vec[n-4..n].iter().copied().collect::<String>() == "mul" {
                   parsing_mode = true;
                   n += 4;
                } else {
                    n += 1;
                }
            }
        }
        Ok(result)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        //TODO: при переборе символов определять do() и doesnt и включать соответствующий режим
        Ok(0)
    }
    
    assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
