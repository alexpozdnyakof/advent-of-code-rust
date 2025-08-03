use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"; 

#[derive(PartialEq, Clone, Debug, Copy)]
enum Operation {
    Add,
    Multiply,
    Concat
}

fn get_op_permutations<T: Copy>(size: usize, variants: &[T]) -> Vec<Vec<T>> {
    let base = variants.len();
    let max_permutations = base.pow(size as u32);
    let mut result: Vec<Vec<T>> = Vec::with_capacity(max_permutations);

    for i in 0..max_permutations {
        let mut n = i;
        let mut permutations: Vec<T> = Vec::with_capacity(size);
        for _ in 0..size {
            permutations.push(variants[n % base]);
            n /= base;
        }
        permutations.reverse();
        result.push(permutations);
    }
    result
}

fn concat_numbers(a: usize, b: usize) -> usize {
        let digits = (b as f64).log10().floor() as u32 + 1;
        let factor = 10_usize.pow(digits);
        a * factor + b 
}


fn main() -> Result<()> {
    start_day(DAY);
    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0; 
        'line: for line in reader.lines().flatten() {
            let splitted: Vec<&str> = line.split(":").collect();
            let expected = splitted[0].parse::<usize>().unwrap();
            let numbers: Vec<usize> = splitted[1].trim().split(" ").map(|s| s.parse::<usize>().unwrap()).collect();
            let pow: usize = if numbers.len() > 0 { numbers.len() - 1 } else { 0 };
            
            for permutation in get_op_permutations(pow, &[Operation::Add, Operation::Multiply]).iter() {
                let mut result: usize = numbers[0];
                for (i, operation) in permutation.iter().enumerate() {
                    match operation {
                        Operation::Add => result += numbers[i+1],
                        Operation::Multiply => result *= numbers[i+1],
                        _ => {}
                    }    
                }
                if result == expected {
                    answer += expected;
                    continue 'line;
                }
            }
        }
        Ok(answer)
    }

    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
   


    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0; 
        'line: for line in reader.lines().flatten() {
            let splitted: Vec<&str> = line.split(":").collect();
            let expected = splitted[0].parse::<usize>().unwrap();
            let numbers: Vec<usize> = splitted[1].trim().split(" ").map(|s| s.parse::<usize>().unwrap()).collect();
            let pow: usize = if numbers.len() > 0 { numbers.len() - 1 } else { 0 };
            for permutation in get_op_permutations(pow, &[Operation::Add, Operation::Multiply, Operation::Concat]).iter() {
                let mut result: usize = numbers[0];
                for (i, operation) in permutation.iter().enumerate() {
                    result = match operation {
                        Operation::Add => result + numbers[i+1],
                        Operation::Multiply => result * numbers[i+1],
                        Operation::Concat => concat_numbers(result, numbers[i+1])
                    };
                }

                if result == expected {
                    answer += expected;
                    continue 'line;
                }
            }
        }
        Ok(answer)
    }
    
    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
