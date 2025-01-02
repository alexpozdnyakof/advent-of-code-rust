use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
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

fn l_char(s: &String) -> char { s.chars().nth(s.len()-1).unwrap() }

fn main() -> Result<()> {
    start_day(DAY);
    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let word = "XMAS";
        let word_rev = word.chars().rev().collect::<String>();
        let is_tail = |ch: char| { ch == word.chars().nth(0).unwrap() || ch == l_char(&word.to_string()) };
        
        let mut result: usize = 0;
        let mut cache: HashMap<(usize,usize), char> = HashMap::new();
        
        for (i, line) in reader.lines().flatten().enumerate() {
            let chars = line.chars().collect::<Vec<_>>();
            let mut n: usize = 0;
            while n < chars.len() {
                 if is_tail(chars[n]) { cache.insert((i,n),chars[n]); };

                 let mut c: usize = 1;
                 let mut buf = vec![chars[n].to_string(); 4];
                 while c < word.len() {
                     if is_tail(chars[n]) && chars.len() - n >= word.len() { buf[0].push(chars[n+c]); }
                     if c <= i { 
                          if (buf[1].len() <= 1 || !is_tail(l_char(&buf[1]))) && cache.contains_key(&(i-c,n)) {
                             buf[1].push(*cache.get(&(i-c,n)).unwrap()); 
                          }
                          if c <= n && (buf[2].len() <= 1 || !is_tail(l_char(&buf[2]))) && cache.contains_key(&(i-c,n-c)){
                             buf[2].push(*cache.get(&(i-c,n-c)).unwrap()); 
                         }
                         if c <= chars.len() - n && (buf[3].len() <= 1 || !is_tail(l_char(&buf[3]))) && cache.contains_key(&(i-c,n+c)) {
                             buf[3].push(*cache.get(&(i-c,n+c)).unwrap()); 
                         }
                     }
                      
                     c += 1;
                 }
                 for (si, s) in buf.iter().map(|buf_str| { buf_str.chars().rev().collect::<String>() }).enumerate() {
                     if s == &word[0..s.len()] || s == &word_rev[0..s.len()]  {
                         if s.len() == word.len() { result +=1; } else if si > 0 { cache.insert((i,n),chars[n]); }
                     }  
                 }
                 n +=1;
            }
            
        }
        Ok(result)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let word = "MAS";
        let word_rev = word.chars().rev().collect::<String>();
        let is_tail = |ch: char| { ch == word.chars().nth(0).unwrap() || ch == l_char(&word.to_string()) };
        let is_middle = |ch: char| { ch ==  word.chars().nth(1).unwrap() };
        let is_equal = |s: String| { s == word || s == word_rev };
        let mut result: usize = 0;
        let mut cache: HashMap<(usize,usize), char> = HashMap::new();
        
        for (i, line) in reader.lines().flatten().enumerate() {
            let chars = line.chars().collect::<Vec<_>>();
            let mut n: usize = 0;
            while n < chars.len() {
                if is_tail(chars[n]) { cache.insert((i,n),chars[n]); };
                if i > 0 && n > 0 {
                    if n < chars.len() && 
                       chars[n] == word.chars().nth(1).unwrap() && 
                       cache.contains_key(&(i-1, n-1)) && 
                       cache.contains_key(&(i-1, n+1)) && 
                       is_tail(*cache.get(&(i-1, n-1)).unwrap()) && is_tail(*cache.get(&(i-1,n+1)).unwrap()) { 
                           cache.insert((i,n),chars[n]);
                       }
                }
                if i >= word.len()-1 && 
                   n >= word.len()-1 && 
                   is_tail(chars[n]) && 
                   is_tail(chars[n-2]) &&
                   cache.contains_key(&(i-1,n-1)) &&
                   cache.contains_key(&(i-2,n-2)) &&
                   cache.contains_key(&(i-2,n))  {
                     let upper_middle = cache.get(&(i-1,n-1)).unwrap();
                     let upper_left = cache.get(&(i-2,n-2)).unwrap();
                     let upper_right = cache.get(&(i-2,n)).unwrap();
                     if is_middle(*upper_middle) && is_tail(*upper_left) && is_tail(*upper_right) {
                          let first_word = format!("{}{}{}",*upper_left, *upper_middle, chars[n]);
                          let second_word = format!("{}{}{}",*upper_right, *upper_middle, chars[n-2]);
                          if i == 2 && n == 3 { println!("{} {}", first_word, second_word)};

                          if is_equal(first_word) && is_equal(second_word) { println!("{} {}", i, n); result += 1 }
                          
                     }

                     // в кэше есть А на i+1 n-1
                     // получить обе диагонали в виде строк
                     // сравнить с исходным словом в обе стороны, если совпадает увеличить результат на 1  
                }
                n += 1; 
            }
        } 
         println!("{:?}", cache);
         //TODO: for i > 1 и S&M ищем свою и смежную диагональ если влазит
         Ok(result)
    }
    
    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
