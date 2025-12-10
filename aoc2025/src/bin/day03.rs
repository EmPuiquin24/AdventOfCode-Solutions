use std::fs;

fn main() {
    let aoc_input = fs::read_to_string("inputs/day03.txt").expect("Document couldn't be read");
    let rows: Vec<&str> = aoc_input.lines().collect();

    let mut sum1 = 0;
    let mut sum2 = 0;

    for row in &rows {
        let max_joltage1 = get_highest_number(row, 2);
        sum1 += max_joltage1;
        
        let max_joltage2 = get_highest_number(row, 12);
        sum2 += max_joltage2;
    }
    println!("The sum joltage 1 is: {}", sum1);
    println!("The sum joltage 2 is: {}", sum2);
}

fn get_highest_number(bank: &str, joltage_len: usize ) -> i64 {
    
    let mut to_remove = bank.len() - joltage_len;

    let mut max_joltage = String::new();
    
    for c in bank.chars() {
        while to_remove > 0 && !max_joltage.is_empty() && max_joltage.chars().last().unwrap() < c {
            max_joltage.pop();
            to_remove -= 1;
        }
        
        max_joltage.push(c);
    }
    
    while to_remove > 0 {
        max_joltage.pop();
        to_remove -= 1;
    }
    
    max_joltage.parse().unwrap()
}

