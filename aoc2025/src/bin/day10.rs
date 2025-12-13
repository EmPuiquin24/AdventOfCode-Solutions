// This solution is literally the Rust implementation of this Python solution XD:
// https://aoc.winslowjosiah.com/solutions/2025/day/10/
use std::fs;
use std::collections::HashMap;

fn main() {
    let aoc_input = fs::read_to_string("inputs/day10.txt").expect("Document couldn't be read");
    let rows: Vec<&str> = aoc_input.lines().filter(|l| !l.trim().is_empty()).collect();

    let mut total_p1: i32 = 0;
    let mut total_p2: i32 = 0;

    for row in rows {
        let (indicators, buttons, joltages) = parse_machine(row);

        let patterns = valid_patterns(&buttons);

        let p1 = configure_indicators(indicators, &patterns).unwrap();
        total_p1 += p1;

        let p2 = configure_joltages(&joltages, &patterns).unwrap();
        total_p2 += p2;
    }

    println!("P1: {}", total_p1);
    println!("P2: {}", total_p2);
}

fn parse_machine(line: &str) -> (u64, Vec<u64>, Vec<i32>) {
    let parts: Vec<&str> = line.split_whitespace().collect();

    let raw_ind = &parts[0][1..parts[0].len() - 1];
    let mut indicators: u64 = 0;
    for (i, ch) in raw_ind.chars().enumerate() {
        if ch == '#' {
            indicators |= 1u64 << i;
        }
    }

    let mut buttons: Vec<u64> = Vec::new();
    for p in &parts[1..parts.len() - 1] {
        let inside = &p[1..p.len() - 1]; 
        let mut mask: u64 = 0;

        for num_str in inside.split(',') {
            let idx: usize = num_str.parse().unwrap();
            mask |= 1u64 << idx;
        }
        buttons.push(mask);
    }

    let raw_j = parts[parts.len() - 1];
    let inside = &raw_j[1..raw_j.len() - 1];
    let joltages: Vec<i32> = inside.split(',').map(|s| s.parse().unwrap()).collect();

    (indicators, buttons, joltages)
}

fn valid_patterns(buttons: &Vec<u64>) -> HashMap<u64, Vec<Vec<u64>>> {
    let mut patterns: HashMap<u64, Vec<Vec<u64>>> = HashMap::new();

    let m = buttons.len();
    let total = 1usize << m;

    for subset in 0..total {
        let mut pattern: u64 = 0;
        let mut presses: Vec<u64> = Vec::new();

        for i in 0..m {
            if ((subset >> i) & 1) == 1 {
                pattern ^= buttons[i];
                presses.push(buttons[i]);
            }
        }

        patterns.entry(pattern).or_insert_with(Vec::new).push(presses);
    }

    patterns
}

fn configure_indicators(indicators: u64, patterns: &HashMap<u64, Vec<Vec<u64>>>) -> Option<i32> {
    let list = patterns.get(&indicators)?;
    let mut best: Option<i32> = None;

    for presses in list {
        let k = presses.len() as i32;
        best = match best {
            None => Some(k),
            Some(b) => Some(b.min(k)),
        };
    }
    best
}

fn configure_joltages(joltages: &Vec<i32>, patterns: &HashMap<u64, Vec<Vec<u64>>>) -> Option<i32> {
    let mut memo: HashMap<Vec<i32>, Option<i32>> = HashMap::new();
    get_min_presses(joltages.clone(), patterns, &mut memo)
}

fn get_min_presses(
    target: Vec<i32>,
    patterns: &HashMap<u64, Vec<Vec<u64>>>,
    memo: &mut HashMap<Vec<i32>, Option<i32>>,
) -> Option<i32> {
    if let Some(v) = memo.get(&target) {
        return *v;
    }

    if target.iter().all(|&x| x == 0) {
        memo.insert(target, Some(0));
        return Some(0);
    }

    let mut indicators: u64 = 0;
    for (i, &val) in target.iter().enumerate() {
        if (val & 1) != 0 {
            indicators |= 1u64 << i;
        }
    }

    let presses_list = match patterns.get(&indicators) {
        Some(v) => v,
        None => {
            memo.insert(target, None);
            return None;
        }
    };

    let mut best: Option<i32> = None;

    for presses in presses_list {
        let mut after = target.clone();
        for &button_mask in presses {
            let mut mask = button_mask;
            while mask != 0 {
                let bit = mask & (!mask + 1); 
                let idx = bit.trailing_zeros() as usize;
                after[idx] -= 1;
                mask &= mask - 1;
            }
        }

        if after.iter().any(|&x| x < 0) {
            continue;
        }

        let mut half: Vec<i32> = Vec::with_capacity(after.len());
        for &x in &after {
            half.push(x / 2);
        }

        let half_ans = get_min_presses(half, patterns, memo);
        if half_ans.is_none() {
            continue;
        }

        let presses_now = presses.len() as i32 + 2 * half_ans.unwrap();
        best = match best {
            None => Some(presses_now),
            Some(b) => Some(b.min(presses_now)),
        };
    }

    memo.insert(target, best);
    best
}

