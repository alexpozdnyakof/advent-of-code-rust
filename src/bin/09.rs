use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::{File};
use std::io::{BufRead, BufReader};

const DAY: &str = "09"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402
";


type RawBlocks = String;
type FileSystem = Vec<isize>;

fn normalize(str: &RawBlocks) -> FileSystem {
    let block_size = |block: char| block.to_digit(10).map(|n| n as usize).unwrap();
    let block_value = |block_id: usize| {
        if block_id % 2 != 0 {
            -1_isize
        } else {
            (block_id / 2) as isize
        }
    };

    str.chars()
        .enumerate()
        .flat_map(|(block_id, block)| vec![block_value(block_id); block_size(block)])
        .collect()
}

fn fragmentate(file_system: &mut FileSystem) -> &mut FileSystem {

    let find_block_right = |file_system: &FileSystem, start_pos: usize|  -> usize {
        let mut i = start_pos;
        while file_system[i] == -1_isize && i != 0 {
            i -= 1;
        }
        i
    };

    let mut sp = 0;
    let mut ep = find_block_right(&file_system, file_system.len() - 1);

    while sp != ep {
        if file_system[sp] == -1_isize {
            file_system.swap(sp, ep);
            ep = find_block_right(&file_system, ep - 1);
        }

        sp += 1;
    }

    file_system
} 



fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        if let Some(raw_blocks) = reader.lines().next().transpose()? {
            let mut file_system = normalize(&raw_blocks);
            let file_system = fragmentate(&mut file_system);
            
            let mut result = 0;
            for (i, b) in file_system.iter().enumerate() {
                if *b == -1_isize { break; }
                result += i * *b as usize; 
            } 
            Ok(result)
        } else {
            Err(anyhow!("File not found"))
        }
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

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
