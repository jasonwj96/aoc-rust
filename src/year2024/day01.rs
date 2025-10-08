use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn parse<P: AsRef<Path>>(path: P) -> io::Result<(Vec<u32>, Vec<u32>)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let nums: Vec<u32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();
        if nums.len() == 2 {
            left.push(nums[0]);
            right.push(nums[1]);
        }
    }

    left.sort();
    right.sort();

    Ok((left, right))
}

pub fn part1(input: &(Vec<u32>, Vec<u32>)) -> io::Result<u32> {
    let (left, right) = input;
    Ok(left.iter()
           .zip(right.iter())
           .map(|(a, b)| a.abs_diff(*b))
           .sum())
}

pub fn part2(input: &(Vec<u32>, Vec<u32>)) -> io::Result<u32> {
    let (left, right) = input;

    let mut freq = HashMap::new();
    for &num in right {
        *freq.entry(num).or_insert(0u32) += 1;
    }

    let score: u32 = left.iter()
                         .map(|&num| num * freq.get(&num).copied().unwrap_or(0))
                         .sum();

    Ok(score)
}