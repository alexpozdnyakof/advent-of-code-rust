use adv_code_2024::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};

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
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    pub fn next(&mut self) -> Self {
        match self {
            Direction::Up => Self::Right,
            Direction::Right => Self::Down,
            Direction::Down => Self::Left,
            Direction::Left => Self::Up,
        } 
    }
}


pub type Coordinates = (usize, usize);
    
#[derive(Debug)]
pub enum MoveError {
    Obstacle,
    End
}

#[derive(Clone)]
pub struct Map {
    pub obstacles: Vec<Coordinates>,
    pub min: usize,
    pub max: usize,
}

impl Default for Map {
    fn default() -> Self {
        Map {
            obstacles: vec![],
            min: 0,
            max: 0,
        }
    }
}
impl Map {
    pub fn try_next(&self, position: Coordinates, direction: &Direction)-> Result<Coordinates, MoveError> {
        let (y,x) = position;
            
        let x_or_y = match direction {
            Direction::Up | Direction::Down => y,
            Direction::Left | Direction::Right => x,
        }; 
            
        if x_or_y == self.max || x_or_y == self.min { 
            return Err(MoveError::End)
        }

        let next_position = match direction {
            Direction::Up => (y-1, x),
            Direction::Down => (y+1, x),
            Direction::Right =>(y, x+1),
            Direction::Left => (y, x-1) 
        };

        if self.obstacles.contains(&next_position) {
            return Err(MoveError::Obstacle)            
        }

        return Ok(next_position)
    }
}

fn main() -> () {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> usize {
        let mut position: Coordinates = (0,0);
        let mut direction = Direction::Up;
        let mut map = Map::default();
        let mut visited: HashSet<Coordinates> = HashSet::new();

        for (i, line) in reader.lines().flatten().enumerate() {
            map.max = i;
            for (j, symbol) in line.chars().enumerate() {
                 if symbol == '#' { map.obstacles.push((i,j)); };
                 if symbol == '^' { 
                     position = (i, j);
                     visited.insert(position);
                 };
            } 
        }
        
        loop {
            match map.try_next(position, &direction) {
                Ok(new_position) => {
                    position = new_position;
                    visited.insert(new_position);
                    continue;
                },
                Err(why) => match why {
                    MoveError::Obstacle => {
                        direction = direction.next();
                        continue;   
                    },
                    MoveError::End => {
                        return visited.len()
                    }
                }
            }
        }
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes())));

    let input_file = BufReader::new(File::open(INPUT_FILE).unwrap());
    let result = time_snippet!(part1(input_file));
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> usize {
        let mut cycles: HashSet<Coordinates> = HashSet::new();
        let mut position: Coordinates = (0,0);
        let mut direction = Direction::Up;
        let mut map = Map::default();
        let mut visited: HashMap<Coordinates, Vec<Direction>> = HashMap::new();

        for (i, line) in reader.lines().flatten().enumerate() {
            map.max = i;
            for (j, symbol) in line.chars().enumerate() {
                 if symbol == '#' { map.obstacles.push((i,j)); };
                 if symbol == '^' { 
                     position = (i, j);
                     visited.insert(position, vec![direction]);
                 };
            } 
        }
        
        loop {
            match map.try_next(position, &direction) {
                Ok(new_position) => {
                    if !visited.contains_key(&new_position){
                        let mut c_pos = position;
                        let mut c_dir = direction;
                        let mut c_visited = visited.clone();
                        let mut c_map = map.clone();
                        c_map.obstacles.push(new_position);
                    
                        loop {
                            match c_map.try_next(c_pos, &c_dir) {
                                Ok(c_new_position) => {
                                    if let Some(already_visited_dir) = c_visited.get_mut(&c_new_position) {
                                        if already_visited_dir.contains(&c_dir) {
                                            cycles.insert(new_position);
                                            break
                                        } else {
                                            already_visited_dir.push(c_dir);
                                        }
                                    } else {
                                        c_visited.insert(c_new_position, vec![c_dir]);
                                    }
                                
                                    c_pos = c_new_position;
                                },
                                Err(why) => match why {
                                    MoveError::Obstacle => {
                                        c_dir = c_dir.next();
                                        continue;
                                    },
                                    MoveError::End => {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    position = new_position;
                    
                    if let Some(already_visited_dir) = visited.get_mut(&new_position) {
                        already_visited_dir.push(direction);
                    } else {
                        visited.insert(new_position, vec![direction]);
                    }
                    continue;
                },
                Err(why) => match why {
                    MoveError::Obstacle => {
                        direction = direction.next();
                        continue;   
                    },
                    MoveError::End => {
                        return cycles.len()
                    }
                }
            }
        }
    }
    
    assert_eq!(6, part2(BufReader::new(TEST.as_bytes())));
    
    let input_file = BufReader::new(File::open(INPUT_FILE).unwrap());
    let result = time_snippet!(part2(input_file));
    println!("Result = {}", result);
    //endregion

    ()
}
