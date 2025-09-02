use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

#[derive(Debug, Eq, Hash, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn get_up_antinode_position(curr: &Point, prev: &Point) -> Point {
    Point {
        x: prev.x - (curr.x - prev.x),
        y: prev.y - (curr.y - prev.y),
    }
}

fn get_bottom_antinode_position(curr: &Point, prev: &Point) -> Point {
    Point {
        x: curr.x + (curr.x - prev.x),
        y: curr.y + (curr.y - prev.y),
    }
}

fn in_field_range(point: &Point, max: usize) -> bool {
    let in_range = |point| point >= 0_isize && point < max as isize;
    in_range(point.x) && in_range(point.y)
}

fn main() -> Result<()> {
    start_day(DAY);
    //region Part 1
    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut broadcasters: HashMap<char, Vec<Point>> = HashMap::new();
        let mut antinodes: HashSet<Point> = HashSet::new();
        for (y, line) in reader.lines().flatten().enumerate() {
            for (x, tag) in line.chars().enumerate() {
                if tag == '.' || tag == '#' {
                    continue;
                }
                let antenna_point = Point {
                    x: x as isize,
                    y: y as isize,
                };
                if let Some(broadcast_list) = broadcasters.get_mut(&tag) {
                    for prev_antenna_point in broadcast_list.iter() {
                        let antinode = get_up_antinode_position(&antenna_point, prev_antenna_point);
                        if in_field_range(&antinode, line.len()) {
                            antinodes.insert(antinode);
                        }

                        let antinode =
                            get_bottom_antinode_position(&antenna_point, prev_antenna_point);
                        if in_field_range(&antinode, line.len()) {
                            antinodes.insert(antinode);
                        }
                    }
                    broadcast_list.push(antenna_point);
                } else {
                    broadcasters.insert(tag, vec![antenna_point]);
                }
            }
        }
        Ok(antinodes.len())
    }

    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut broadcasters: HashMap<char, Vec<Point>> = HashMap::new();
        let mut antinodes: HashSet<Point> = HashSet::new();
        for (y, line) in reader.lines().flatten().enumerate() {
            for (x, tag) in line.chars().enumerate() {
                if tag == '.' || tag == '#' {
                    continue;
                }

                let antenna_point = Point {
                    x: x as isize,
                    y: y as isize,
                };

                if let Some(broadcast_list) = broadcasters.get_mut(&tag) {
                    antinodes.insert(antenna_point);

                    for prev_antenna_point in broadcast_list.iter() {
                        antinodes.insert(*prev_antenna_point);

                        let mut antinode =
                            get_up_antinode_position(&antenna_point, prev_antenna_point);
                        let mut prev_antinode = *prev_antenna_point;

                        while in_field_range(&antinode, line.len()) {
                            antinodes.insert(antinode);
                            let new_antinode = get_up_antinode_position(&prev_antinode, &antinode);

                            prev_antinode = antinode;
                            antinode = new_antinode;
                        }

                        antinode = get_bottom_antinode_position(&antenna_point, prev_antenna_point);
                        prev_antinode = antenna_point;

                        while in_field_range(&antinode, line.len()) {
                            antinodes.insert(antinode);
                            let new_antinode =
                                get_bottom_antinode_position(&antinode, &prev_antinode);

                            prev_antinode = antinode;
                            antinode = new_antinode;
                        }
                    }

                    broadcast_list.push(antenna_point);
                } else {
                    broadcasters.insert(tag, vec![antenna_point]);
                }
            }
        }
        Ok(antinodes.len())
    }

    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
