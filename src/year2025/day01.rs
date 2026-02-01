use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn parse<P: AsRef<Path>>(path: P) -> io::Result<Vec<(char, u32)>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        let value: u32 = chars.as_str().parse().unwrap();

        result.push((dir, value));
    }

    Ok(result)
}

pub fn part1(input: &[(char, u32)]) -> u32 {
    let mut dial: i32 = 50;
    let mut result = 0;

    for &(dir, steps) in input {
        let steps = steps as i32;

        dial += match dir {
            'R' => steps,
            'L' => -steps,
            _ => 0,
        };

        if dial.rem_euclid(100) == 0 {
            result += 1;
        }
    }

    result
}

pub fn part2(input: &[(char, u32)]) -> i32 {
    let mut dial: i32 = 50;
    let mut result: i32 = 0;

    for &(dir, steps) in input {
        for _ in 0..steps {
            match dir {
                'R' => dial = (dial + 1) % 100,
                'L' => dial = (dial + 99) % 100,
                _ => {}
            }

            if dial == 0 {
                result += 1;
            }
        }
    }

    result
}