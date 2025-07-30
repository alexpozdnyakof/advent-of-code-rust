use std::collections::HashMap;
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
     Up,
     Right,
     Down,
     Left
}

type Coordinates = (usize, usize);

fn next_position(position: Coordinates, direction: Direction) -> Coordinates {
    match direction {
        Direction::Up => (position.0-1, position.1),
        Direction::Down => (position.0+1, position.1),
        Direction::Right =>(position.0, position.1 + 1),
        Direction::Left => (position.0, position.1 - 1) 
    }
}

static DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Right, Direction::Down, Direction::Left];

fn next_direction(direction: Direction) -> Direction {
    let cur_idx = DIRECTIONS.iter().position(|dir| *dir == direction).unwrap();    
    if cur_idx == DIRECTIONS.len() - 1 { 
        DIRECTIONS[0] 
    } else { 
        DIRECTIONS[cur_idx + 1]
    }
} 


fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut constraints: Coordinates = (0,0);
        let mut obstacles: Vec<Coordinates> = Vec::new();
        let mut position = (0,0);
        let mut direction = Direction::Up;
        let mut visited: Vec<Coordinates> = Vec::new();
        for (i, line) in reader.lines().flatten().enumerate() {
            constraints.1 = i;
            for (j, symbol) in line.chars().enumerate() {
                 if symbol == '#' { obstacles.push((i,j)); };
                 if symbol == '^' { position = (i, j); };
            } 
        }
        let mut visited_unique_count = 1;
         
        visited.push(position);
        loop {
            let new_position = next_position(position, direction); 
            
            if obstacles.contains(&new_position) {
                 direction = next_direction(direction);
                 continue;
            }
            
            if !visited.contains(&new_position) { visited_unique_count += 1; }
            visited.push(new_position);
            
            let (min,max) = constraints;
            if new_position.0 == max || new_position.1 == max || new_position.0 == min || new_position.1 == min {
                 break
            }       
            
            position = new_position;
        }

        Ok(visited_unique_count)
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    //fn part2<R: BufRead>(reader: R) -> Result<usize> {
        //let mut visited: HashMap<(usize,usize), Vec<Direction>> = HashMap::new();
    //    Ok(6)
    //}
    //
    //assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);
    //
    //let input_file = BufReader::new(File::open(INPUT_FILE)?);
    //let result = time_snippet!(part2(input_file)?);
    //println!("Result = {}", result);
    //endregion

    Ok(())
}
