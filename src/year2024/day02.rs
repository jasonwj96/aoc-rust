use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::Path;

pub fn parse<P: AsRef<Path>>(path: P) -> io::Result<(Vec<Vec<u32>>)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);


    Ok(Vec::new())
}

pub fn part1(input: &Vec<Vec<u32>>) -> u32 {
    0
}


pub fn part2(input: &Vec<Vec<u32>>) -> u32 {
    0
}