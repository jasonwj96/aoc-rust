use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn parse<P: AsRef<Path>>(path: P) -> io::Result<Vec<Vec<u32>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut output = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let nums: Vec<u32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        output.push(nums);
    }

    Ok(output)
}

fn is_safe(row: &[u32]) -> bool {
    if row.len() < 2 {
        return true;
    }

    let diffs: Vec<i32> = row.windows(2).map(|w| w[1] as i32 - w[0] as i32).collect();
    
    let all_increasing = diffs.iter().all(|&d| d >= 1 && d <= 3);
    let all_decreasing = diffs.iter().all(|&d| d <= -1 && d >= -3);

    all_increasing || all_decreasing
}

pub fn part1(input: &Vec<Vec<u32>>) -> u32 {
    input.iter()
         .filter(|row| is_safe(row))
         .count() as u32
}

pub fn part2(input: &Vec<Vec<u32>>) -> u32 {
    input
        .iter()
        .filter(|&row| {
            if is_safe(row) {
                true
            } else {
                // Try removing one element
                (0..row.len()).any(|i| {
                    let mut reduced = row.clone();
                    reduced.remove(i);
                    is_safe(&reduced)
                })
            }
        })
        .count() as u32
}