// These solutions take inspiration in https://github.com/Grazen0/advent-of-code-c/blob/main/src/2025/day_08_1.c
// and https://github.com/Grazen0/advent-of-code-c/blob/main/src/2025/day_08_2.c
// :'v

use std::fs;

fn main() {
    let aoc_input = fs::read_to_string("inputs/day06.txt").expect("Document couldn't be read");
    let lines: Vec<&str> = aoc_input.lines().collect();

    println!("Part 1: {}", p1(&lines));
    println!("Part 2: {}", p2(&lines));
}

fn p1(lines: &Vec<&str>) -> i128 {
    let rows = lines.len();
    let ops_line = lines[rows - 1];

    let ops: Vec<char> = ops_line
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect();

    let cols = ops.len();

    let mut results: Vec<i128> = vec![0; cols];
    for j in 0..cols {
        results[j] = if ops[j] == '+' { 0 } else { 1 };
    }

    for i in 0..rows - 1 { 
        let nums: Vec<i128> = lines[i].split_whitespace().map(|s| s.parse::<i128>().unwrap()).collect(); // :'v

        for j in 0..cols {
            if ops[j] == '+' {
                results[j] += nums[j];
            } else {
                results[j] *= nums[j];
            }
        }
    }

    results.into_iter().sum()
}


fn p2(lines: &Vec<&str>) -> i128 {
    let rows = lines.len();

    let mut cols: usize = 0;
    for line in lines.iter() {
        cols = cols.max(line.len());
    }

    let bytes: Vec<Vec<u8>> = lines.iter().map(|s| s.as_bytes().to_vec()).collect();

    let mut sum: i128 = 0;
    let mut nums: Vec<i128> = Vec::new();

    for j in (0..cols).rev() {
        let mut n: i128 = 0;
        let mut op: u8 = 0;
        let mut col_empty = true;

        for i in 0..rows {
            let line = &bytes[i];
            
            if j >= line.len() { 
                continue; 
            }
    
            let ch = line[j];

            if ch >= b'0' && ch <= b'9' {
                col_empty = false;
                n = (10 * n) + (ch - b'0') as i128;
            } else if ch != b' ' {
                op = ch;
                break;
            }
        }

        if !col_empty {
            nums.push(n);
        }

        if op != 0 {
            let mut result: i128 = if op == b'+' { 0 } else { 1 };

            for v in nums.iter() {
                if op == b'+' { 
                    result += *v;
                }
                else {
                    result *= *v;
                }
            }
            sum += result;
            nums.clear();
        }
    }

    sum
}


