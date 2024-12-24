use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

fn get_line_char(line: String, n: usize) -> char {
    line.chars().nth(n).unwrap()
}

fn main() -> Result<()> {
    start_day(DAY);
    const XMAS: &str = "XMAS";
    const XMAS_REV: &str = "SAMX";
    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines().flatten().collect::<Vec<_>>();
        let mut result: usize = 0;
        let mut buf_cache: Vec<Vec<char>> = Vec::new();
        for (i, line) in lines.into_iter().enumerate() {
            let chars = line.chars().collect::<Vec<_>>();
            let mut n: usize = 0;
            
            while n < chars.len() {
                if chars[n] == 'X' || chars[n] == 'S'  {
                    if chars.len() - n > 3 {
                       let x_str = chars_slice_to_str(&chars[n..n+4]); 
                       if x_str == XMAS || x_str == XMAS_REV {
                           result += 1;
                       }
                    }
                    if buf_cache.len() >= XMAS.len() - 1 {
                       {
                           let x_str = chars_slice_to_str(vec![chars[n], buf_cache[buf_cache.len()-1][n], buf_cache[buf_cache.len()-2][n], buf_cache[buf_cache.len()-3][n]].as_slice());
                           if x_str == XMAS || x_str == XMAS_REV {
                               result += 1;
                           }
                       }
        
                       if n > 2 {
                          let x_str = chars_slice_to_str(vec![chars[n], buf_cache[buf_cache.len()-1][n-1], buf_cache[buf_cache.len()-2][n-2], buf_cache[buf_cache.len()-3][n-3]].as_slice());
                          if x_str == XMAS || x_str == XMAS_REV {
                              result += 1;
                          }
                       }
                       if chars.len() - n > 3 {
                          let x_str = chars_slice_to_str(vec![chars[n], buf_cache[buf_cache.len()-1][n+1], buf_cache[buf_cache.len()-2][n+2], buf_cache[buf_cache.len()-3][n+3]].as_slice());
                          if x_str == XMAS || x_str == XMAS_REV {
                             result += 1;
                          }     
                       }  	
                    } 
                }
                n += 1;   
            } 
            
            buf_cache.push(chars);
            
        }
        Ok(result)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

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
