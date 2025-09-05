use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402
";

type RawBlocks = String;

#[derive(Clone, Copy, Debug)]
struct LocationDescriptor {
    size: usize,
    id: Option<usize>,
}

type Position = usize;

struct LocationIndex {
    index: BTreeMap<Position, LocationDescriptor>,
}

impl LocationIndex {
    fn new() -> Self {
        LocationIndex {
            index: BTreeMap::new(),
        }
    }

    fn insert(&mut self, position: usize, descriptor: LocationDescriptor) {
        let mut new_start = position;
        let mut new_size = descriptor.size;

        if let Some((&prev_position, prev)) = self.index.range(..=position).last() {
            if prev_position + prev.size == position && prev.id == descriptor.id {
                new_start = prev_position;
                new_size += prev.size;
                self.index.remove(&prev_position);
            }
        }

        if let Some((&next_position, next)) = self.index.range(position..).next() {
            if new_start + new_size == next_position && next.id == descriptor.id {
                new_size += next.size;
                self.index.remove(&next_position);
            }
        }

        self.index.insert(
            new_start,
            LocationDescriptor {
                id: descriptor.id,
                size: new_size,
            },
        );
    }

    fn log(&self) {
        for (&k, &v) in self.index.iter() {
            println!("{:?}: {:?}\n", k, v);
        }
    }
}

fn create_free_desc(size: usize) -> LocationDescriptor {
    LocationDescriptor { id: None, size }
}

fn create_data_desc(size: usize, id: usize) -> LocationDescriptor {
    LocationDescriptor { id: Some(id), size }
}

fn from_raw(raw: &RawBlocks) -> FileSystem {
    let mut ntfs = FileSystem::new();
    let mut offset = 0;
    for (i, ch) in raw.chars().enumerate() {
        if let Some(size) = ch.to_digit(10).map(|n| n as usize) {
            if i % 2 == 0 {
                ntfs.files
                    .index
                    .insert(offset, create_data_desc(size, i / 2));
            } else {
                ntfs.free_space.index.insert(offset, create_free_desc(size));
            }
            offset += size;
        } else {
            continue;
        }
    }

    ntfs
}

struct FileSystem {
    files: LocationIndex,
    free_space: LocationIndex,
}

impl FileSystem {
    fn new() -> Self {
        FileSystem {
            files: LocationIndex::new(),
            free_space: LocationIndex::new(),
        }
    }

    fn free(&mut self, target: usize, size: usize) -> Result<(usize, usize)> {
        if let Some((&file_position, file)) = self.files.index.range(..=&target).last() {
            if size == file.size {
                self.free_space
                    .insert(file_position, create_free_desc(file.size));
                self.files.index.remove(&file_position);
            } else {
                if let Some(file_id) = file.id {
                    let new_file = create_data_desc(file.size - size, file_id);
                    let free_space = create_free_desc(size);

                    self.free_space
                        .insert(file_position + new_file.size, free_space);

                    self.files.insert(file_position, new_file);
                }
            }
            Ok((file_position, size))
        } else {
            Err(anyhow!("File: {} not found", target))
        }
    }

    fn copy(&mut self, file_position: usize, size: usize, allocated_position: usize) {
        if let Some(file) = self.files.index.get(&file_position) {
            if file.size == size {
                self.files.insert(allocated_position, *file);
            } else {
                if let Some(file_id) = file.id {
                    self.files
                        .insert(allocated_position, create_data_desc(size, file_id));
                }
            }
        }
    }

    fn check_sum(&self) -> usize {
        self.files
            .index
            .iter()
            .fold(0, |mut acc, (&position, &file)| {
                for i in 0..file.size {
                    if let Some(file_id) = file.id {
                        acc += (position + i) * file_id;
                    }
                }
                acc
            })
    }

    fn allocate_before(&mut self, max_position: &usize, size: usize) -> Option<usize> {
        for (&position, &free) in self.free_space.index.range(..=max_position) {
            if free.size >= size {
                self.free_space.index.remove(&position);
                if free.size > size {
                    self.free_space
                        .insert(&position + size, create_free_desc(free.size - size));
                }
                return Some(position);
            }
        }

        None
    }

    fn files(&self) -> &BTreeMap<usize, LocationDescriptor> {
        &self.files.index
    }

    fn print(&self) {
        let mut e_index = self.files.index.clone();
        e_index.extend(self.free_space.index.clone());
        let chars: String = e_index
            .iter()
            .flat_map(|(_, desc)| {
                vec![
                    if let Some(id) = desc.id {
                        id.to_string()
                    } else {
                        String::from('.')
                    };
                    desc.size
                ]
            })
            .collect();
        println!("{}", chars);
    }
}

fn fragmentate(
    position: usize,
    file: LocationDescriptor,
    file_system: &mut FileSystem,
) -> Result<&mut FileSystem> {
    let mut size = file.size;
    let chunk_size = 1;
    while size != 0 {
        if let Some(allocated_position) = file_system.allocate_before(&position, chunk_size) {
            file_system.copy(position, chunk_size, allocated_position);
            file_system.free(position, chunk_size)?;
            size -= chunk_size;
        } else {
            return Err(anyhow!("Unable allocate free space"));
        }
    }
    Ok(file_system)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        if let Some(raw_blocks) = reader.lines().next().transpose()? {
            let mut file_system = from_raw(&raw_blocks);
            let files: Vec<(usize, LocationDescriptor)> =
                file_system.files().iter().map(|(&k, &v)| (k, v)).collect();

            for (position, file) in files.iter().rev() {
                if let Err(reason) = fragmentate(*position, *file, &mut file_system) {
                    println!("{:?}", reason);
                    break;
                }
            }
            Ok(file_system.check_sum())
        } else {
            Err(anyhow!("File not found"))
        }
    }

    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        Ok(0)
    }

    assert_eq!(2858, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
