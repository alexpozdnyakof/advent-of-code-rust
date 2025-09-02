use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

fn collect_array<T, I, const N: usize>(itr: I) -> [T; N]
where
    T: Default + Copy,
    I: IntoIterator<Item = T>,
{
    let mut res = [T::default(); N];
    for (it, elem) in res.iter_mut().zip(itr) {
        *it = elem
    }

    res
}

struct Graph {
    data: HashMap<usize, Vec<usize>>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            data: HashMap::new(),
        }
    }
    fn add_edge(&mut self, v: usize, e: usize) {
        if self.data.contains_key(&v) {
            self.data.get_mut(&v).unwrap().push(e);
        } else {
            self.data.insert(v, vec![e]);
        }
    }
    fn validate_edge(&self, v: &usize, e: &usize) -> bool {
        self.data.contains_key(&v) && self.data.get(&v).unwrap().contains(&e)
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut is_parsing_finished = false;
        let mut graph = Graph::new();
        let mut result: usize = 0;
        for line in reader.lines().flatten() {
            if line.len() == 0 {
                is_parsing_finished = true;
                continue;
            }
            if !is_parsing_finished {
                let [v, e]: [usize; 2] =
                    collect_array(line.split("|").map(|s| s.parse::<usize>().unwrap()));
                graph.add_edge(v, e);
            } else {
                let path: Vec<usize> = line
                    .split(",")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();

                for (i, edge) in (&path).into_iter().enumerate() {
                    if i == path.len() - 1 {
                        result += path[(path.len() - 1) / 2];
                        break;
                    }
                    if !graph.validate_edge(edge, &path[i + 1]) {
                        break;
                    }
                }
            }
        }
        Ok(result)
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut is_parsing_finished = false;
        let mut graph = Graph::new();
        let mut result: usize = 0;

        for line in reader.lines().flatten() {
            if line.len() == 0 {
                is_parsing_finished = true;
                continue;
            }
            if !is_parsing_finished {
                let [v, e]: [usize; 2] =
                    collect_array(line.split("|").map(|s| s.parse::<usize>().unwrap()));
                graph.add_edge(v, e);
            } else {
                let mut path: Vec<usize> = line
                    .split(",")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();

                let mut i = 0;
                let mut swap_count = 0;
                while i < path.len() {
                    if i == path.len() - 1 {
                        if swap_count > 0 {
                            result += path[(path.len() - 1) / 2]
                        };
                        break;
                    }
                    if graph.validate_edge(&path[i], &path[i + 1]) {
                        i += 1;
                        continue;
                    }

                    if graph.validate_edge(&path[i + 1], &path[i]) {
                        path.swap(i, i + 1);
                        swap_count += 1;
                        i = if i == 0 { i } else { i - 1 };
                    } else {
                        break;
                    }
                }
            }
        }
        println!("{}", result);
        Ok(result)
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
