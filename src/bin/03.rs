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

fn extract_multiply_result(outer_step: usize, chars: &[char]) -> (usize, usize) {
    let mut x = outer_step;
    let mut cache = String::new();
    while x < chars.len() {
        if chars[x].is_numeric() || chars[x] == ',' {
            cache.push(chars[x]);
            x += 1;
        } else { 
            if chars[x] != ')' { cache = String::new(); }
            x += 1;    
            break;   
        }          
    }
    let result: usize = if cache.len() > 0 { cache.split(",").fold(1, |acc, d| { acc * d.parse::<usize>().unwrap()}) } else { 0 }; 
    (x, result)

}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead + std::fmt::Debug>(reader: R) -> Result<usize> {
        let answer = reader.lines();
        let mut result = 0;
        for line in answer.into_iter() {
            let chars: Vec<_> = line.unwrap().chars().collect();
            
            let mut n: usize = 0; 
            while n < chars.len() {
                if chars[n].is_numeric() && n > 3  && chars_slice_to_str(&chars[n-4..n]) == "mul(" {
                    let (shift, step_result) = extract_multiply_result(n, &chars);
                    result += step_result;
                    n = shift;
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
                //if n <= chars.len() - 4 && chars_slice_to_str(&chars[n..n+4]) == "do()" {
                //if n <= chars.len() - 7 && chars_slice_to_str(&chars[n..n+7]) == "don't()" {
        Ok(0)
    }
    
    assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
