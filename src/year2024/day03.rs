use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn parse<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut output = Vec::new();

    for line in reader.lines() {
        output.push(line?);
    }

    Ok(output)
}

pub fn part1(input: &Vec<String>) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut output: u32 = 0;

    for line in input {
        for caps in re.captures_iter(line) {
            let a: u32 = caps[1].parse().unwrap();
            let b: u32 = caps[2].parse().unwrap();
            output += a * b;
        }
    }

    output
}

pub fn part2(input: &Vec<String>) -> u32 {
    0
}