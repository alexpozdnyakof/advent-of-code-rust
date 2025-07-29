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

#[derive(Debug)]
struct GuardState {
    direction: Direction,
    _turns: Vec<Coordinates>
}


impl GuardState {
    fn new(init_position: Coordinates) -> GuardState {
         GuardState { 
            direction: Direction::Up,
            _turns:vec![init_position],
         }
    }

    fn change_direction(&mut self) -> Direction {
        static DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Right, Direction::Down, Direction::Left];
        let cur_idx = DIRECTIONS.iter().position(|dir| dir == &self.direction).unwrap();
        
        if cur_idx == DIRECTIONS.len() - 1 { 
           self.direction = DIRECTIONS[0]; 
        } else { 
           self.direction = DIRECTIONS[cur_idx + 1]
        };
        self.direction
    }

    fn go_forward(&mut self) -> Coordinates {
        let mut new_position = self._turns[self._turns.len()-1];
        new_position = match self.direction {
            Direction::Up => (new_position.0-1, new_position.1),
            Direction::Down => (new_position.0+1, new_position.1),
            Direction::Right =>(new_position.0, new_position.1 + 1),
            Direction::Left => (new_position.0, new_position.1 - 1) 
        };
        if let Some(idx) = self._turns.iter().position(|item| { item == &new_position}) {
            self._turns.remove(idx);
        }
        self._turns.push(new_position);
        new_position
    }
    
    fn go_back(&mut self) {
        self._turns.pop();
    }
    fn position(&self) -> Option<&Coordinates>{
        self._turns.last()
    }
    fn count_turns(&self) -> usize {
        self._turns.len()
    }
}


fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut constraints: Coordinates = (0,0);
        let mut obstacles: Vec<Coordinates> = Vec::new();
        let mut start_position = (0,0); 
        
        for (i, line) in reader.lines().flatten().enumerate() {
            constraints.1 = i;
            for (j, symbol) in line.chars().enumerate() {
                 if symbol == '#' { obstacles.push((i,j)) };
                 if symbol == '^' { start_position = (i, j) };
            } 
        }
        let mut guard = GuardState::new(start_position);

        loop {
            let (x,y) = *guard.position().unwrap();
            let (min,max) = constraints;
            if x == max || y == max || x == min || y == min {
                 break
            }       

            guard.go_forward();
            
            if let Some(next_position) = guard.position() {
                 if obstacles.contains(&next_position) { 
                      guard.go_back();
                      guard.change_direction();
                      continue;
                 }
            }
        }

        Ok(guard.count_turns())
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
