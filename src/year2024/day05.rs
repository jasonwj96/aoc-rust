use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn parse<P: AsRef<Path>>(path: P)
                             -> io::Result<(Vec<(u32, u32)>, Vec<Vec<u32>>)>
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    let mut in_updates = false;

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            in_updates = true;
            continue;
        }
        if !in_updates {
            // rule line: "X|Y"
            if let Some(pos) = line.find('|') {
                let left = line[..pos].parse::<u32>().unwrap();
                let right = line[pos + 1..].parse::<u32>().unwrap();
                rules.push((left, right));
            }
        } else {
            // update line: "a,b,c,..."
            let pages: Vec<u32> = line.split(',')
                                      .map(|s| s.trim().parse().unwrap())
                                      .collect();
            updates.push(pages);
        }
    }

    Ok((rules, updates))
}

// Check if an update is valid (i.e., respects all applicable rules)
fn is_valid_update(update: &Vec<u32>, rules: &[(u32, u32)]) -> bool {
    // map page -> index in update
    let mut pos = HashMap::new();
    for (idx, &page) in update.iter().enumerate() {
        pos.insert(page, idx);
    }
    for &(a, b) in rules {
        if pos.contains_key(&a) && pos.contains_key(&b) {
            // rule applies
            if pos[&a] >= pos[&b] {
                return false;
            }
        }
    }
    true
}

// Fix an invalid update by reordering it to satisfy the rules.
fn fix_update(mut update: Vec<u32>, rules: &[(u32, u32)]) -> Vec<u32> {
    // naive method: repeatedly scan through rules, if a rule (a|b) is violated
    // within this update (i.e., b occurs before a), swap them and continue until stable.
    // This should terminate because for each update the constraints are resolvable.
    let mut changed = true;
    while changed {
        changed = false;
        let pos_map: HashMap<u32, usize> = update.iter().enumerate()
                                                 .map(|(i, &v)| (v, i))
                                                 .collect();
        for &(a, b) in rules {
            if let (Some(&pa), Some(&pb)) = (pos_map.get(&a), pos_map.get(&b)) {
                if pa >= pb {
                    // swap positions of a and b
                    let idx_a = pa;
                    let idx_b = pb;
                    update.swap(idx_a, idx_b);
                    changed = true;
                }
            }
        }
    }
    update
}

pub fn part1(rules: &[(u32, u32)], updates: &Vec<Vec<u32>>) -> u64 {
    updates.iter()
           .filter(|upd| is_valid_update(upd, rules))
           .map(|upd| {
               let mid = upd.len() / 2;
               upd[mid] as u64
           })
           .sum()
}

pub fn part2(rules: &[(u32, u32)], updates: &Vec<Vec<u32>>) -> u64 {
    updates.iter()
           .map(|upd| {
               if is_valid_update(upd, rules) {
                   upd.clone()
               } else {
                   fix_update(upd.clone(), rules)
               }
           })
           .map(|upd2| {
               let mid = upd2.len() / 2;
               upd2[mid] as u64
           })
           .sum()
}