use std::fs;
use regex::Regex;

fn main() {
    let aoc_input = fs::read_to_string("inputs/day02.txt").expect("Document couldn't be read");
    let ranges_regex = Regex::new(r"(\d+)-(\d+)").unwrap();

    let mut i_ids_sum: i64 = 0;
    let mut i_ids_sum2: i64 = 0;

    for range in ranges_regex.captures_iter(&aoc_input) {
        let f_num: i64 = range[1].parse().unwrap();
        let l_num: i64 = range[2].parse().unwrap();

        for n in f_num..l_num+1 {
            if is_nn1(n) {
                i_ids_sum += n;
            }

            if is_nn2(n) {
                i_ids_sum2 += n;
            }
        }
    }

    println!("Invalid IDs sum: {}", i_ids_sum);
    println!("Invalid IDs sum part 2: {}", i_ids_sum2);
}

fn is_nn1(n: i64) -> bool {
    let s = n.to_string();
    let len = s.len();

    if len % 2 != 0 {
        return false;
    }

    let mid = len / 2;
    let (a, b) = s.split_at(mid);

    a == b
}

fn is_nn2(n: i64) -> bool {
    let s = n.to_string();

    let len = s.len();

    for size in 1..=len/2 {

        if len % size == 0 {
            let pattern = &s[0..size];
            let reps = len / size;

            if reps >= 2 && pattern.repeat(reps) == s {
                return true;
            }
        }
    }

    false
}
