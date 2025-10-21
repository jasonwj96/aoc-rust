use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn parse<P: AsRef<Path>>(path: P) -> io::Result<Vec<(i64, Vec<i64>)>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut output = Vec::new();
    for line_res in reader.lines() {
        let line = line_res?;
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 { continue; }
        let target = parts[0].trim().parse::<i64>()
                             .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let nums: Vec<i64> = parts[1].trim().split_whitespace()
                                     .map(|s| s.parse::<i64>()
                                               .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)))
                                     .collect::<Result<_,_>>()?;
        output.push((target, nums));
    }
    Ok(output)
}

fn can_reach(nums: &[i64], idx: usize, current: i64, target: i64, allow_concat: bool) -> bool {
    if idx == nums.len() {
        return current == target;
    }
    let next = nums[idx];
    // Option: plus
    if can_reach(nums, idx+1, current + next, target, allow_concat) {
        return true;
    }
    // Option: multiply
    if can_reach(nums, idx+1, current * next, target, allow_concat) {
        return true;
    }
    // Option: concat (only if allowed)
    if allow_concat {
        // compute number of digits in next
        let mut tmp = next;
        let mut digits = 0;
        if tmp == 0 { digits = 1; }
        while tmp > 0 {
            digits += 1;
            tmp /= 10;
        }
        let new_val = current * 10_i64.pow(digits as u32) + next;
        if can_reach(nums, idx+1, new_val, target, allow_concat) {
            return true;
        }
    }
    false
}

pub fn part1(data: &[(i64, Vec<i64>)]) -> i64 {
    data.iter()
        .filter_map(|&(target, ref nums)| {
            if nums.is_empty() {
                None
            } else if can_reach(&nums, 1, nums[0], target, false) {
                Some(target)
            } else {
                None
            }
        })
        .sum()
}

pub fn part2(data: &[(i64, Vec<i64>)]) -> i64 {
    data.iter()
        .filter_map(|&(target, ref nums)| {
            if nums.is_empty() {
                None
            } else if can_reach(&nums, 1, nums[0], target, true) {
                Some(target)
            } else {
                None
            }
        })
        .sum()
}