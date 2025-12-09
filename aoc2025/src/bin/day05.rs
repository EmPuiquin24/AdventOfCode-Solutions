use std::fs;
use regex::Regex;

fn main() {
    let aoc_input = fs::read_to_string("inputs/day05.txt").expect("Document couldn't be read");
    let ingre_regex = Regex::new(r"(?m)^(\d+)-(\d+)$").unwrap();
    let ingre_ids = Regex::new(r"(?m)^(\d+)$").unwrap(); 

    let mut ingre_ranges: Vec<Vec<i64>> = Vec::new();

    let mut fresh_counter: i64 = 0;
    let mut fresh_counter2: i64 = 0;

    for range in ingre_regex.captures_iter(&aoc_input) {
        let a: i64 = range[1].parse().unwrap();
        let b: i64 = range[2].parse().unwrap();
        
        let mut ingre_range: Vec<i64> = Vec::new();
        ingre_range.push(a);
        ingre_range.push(b);

        if ingre_ranges.len() == 0 {
            ingre_ranges.push(ingre_range.clone());
            continue;
        }
        
        let mut new_range = ingre_range.clone();
        let mut i = 0;

        while i < ingre_ranges.len() {
            let x = &ingre_ranges[i];

            if !out_range(&new_range, x) {
                new_range = get_new_range(&new_range, x);
                ingre_ranges.remove(i);
            } else {
                i += 1;
            }

        }
        ingre_ranges.push(new_range);

    }

    for id in ingre_ids.captures_iter(&aoc_input) {
        let id_num: i64 = id[1].parse().unwrap();

        for x in &ingre_ranges {
            let a: i64 = x[0];
            let b: i64 = x[1];
            if id_num >= a && id_num <= b {
                fresh_counter += 1;
                break;
            }
        }
    }

    // Part 2
    for x in &ingre_ranges {
        let a: i64 = x[0];
        let b: i64 = x[1];

        fresh_counter2 += b - a + 1;
    }

    println!("The fresh ingredients amount is: {}", fresh_counter);
    println!("The fresh ingredients amount for p2 is: {}", fresh_counter2);
}


fn out_range(v1: &Vec<i64>, v2:&Vec<i64>) -> bool {
    let a = v1[0]; let b = v1[1];
    let g = v2[0]; let h = v2[1];
    let mut r = false;

    if h < a || b < g {
        r = true;
    }
    r
}

fn get_new_range(v1: &Vec<i64>, v2:&Vec<i64>) -> Vec<i64> {
    let a = v1[0]; let b = v1[1];
    let g = v2[0]; let h = v2[1];

    let mut new_range: Vec<i64> = Vec::new(); 

    if g < a {
        if b < h {
            new_range.push(g);
            new_range.push(h);
        }
        else {
            new_range.push(g);
            new_range.push(b);
        }
    }
    else {
        if b < h {
            new_range.push(a);
            new_range.push(h);
        }
        else {
            new_range.push(a);
            new_range.push(b);
        }
    }
    new_range
}

