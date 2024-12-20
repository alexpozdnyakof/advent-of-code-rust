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
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead + std::fmt::Debug>(reader: R) -> Result<usize> {
        let answer = reader.lines();
        let mut result = 0;
        for line in answer.into_iter() {
            let chars: Vec<_> = line.unwrap().chars().collect();
            let mut parsing_mode: bool = false;
            let mut numbers_to_multiply = String::new();

            let mut n: usize = 0; 
            while n < chars.len() {
                if parsing_mode {
                   if chars[n].is_numeric() || chars[n] == ',' { 
                       numbers_to_multiply.push(chars[n]);
                   } else {
                       if chars[n] == ')' { 
                           let multiply_result = numbers_to_multiply.split(",").fold(1, |acc, v| {
                               acc * v.parse::<usize>().unwrap()
                           });
                           result += multiply_result;
                       }
                       numbers_to_multiply = String::new();
                       parsing_mode = false;
                   }
                }
                if n <= chars.len() - 4  && chars_slice_to_str(&chars[n..n+4]) == "mul(" {
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
