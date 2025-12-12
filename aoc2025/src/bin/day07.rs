use std::fs;

fn main() {
    
    let aoc_input = fs::read_to_string("inputs/day07.txt").expect("Document couldn't be read");
    
    let grid: Vec<Vec<char>> = aoc_input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let columns = grid[0].len();
    
    let mut start_col: usize = 0;
    for c in 0..columns {
        if grid[0][c] == 'S' {
            start_col = c;
            break;
        }
    }
    
    let p1 = count_splits(&grid, rows, columns, start_col);
    let p2 = count_timelines(&grid, rows, columns, start_col);

    println!("P1 - The amount of the beam splitted is: {}", p1);
    println!("P2 - The amount of times is: {}", p2);
}

fn count_splits(grid: &Vec<Vec<char>>, rows: usize, columns: usize, start_col: usize) -> i64 {
    let mut positions: Vec<usize> = vec![start_col];
    let mut split_counter: i64 = 0;

    for i in 0..rows {
        let mut j = 0;
        while j < positions.len() {
            let col = positions[j];

            if grid[i][col] == '^' {
                split_counter += 1;
                positions.remove(j);

                if col > 0 {
                    let left_split = col - 1;
                    if !positions.contains(&left_split) {
                        positions.push(left_split);
                    }
                }

                if col + 1 < columns {
                    let right_split = col + 1;
                    if !positions.contains(&right_split) {
                        positions.push(right_split);
                    }
                }

            } else {
                j += 1;
            }
        }
    }

    split_counter
}

// The part 2 solution is inspired on:
// https://github.com/Grazen0/advent-of-code-c/blob/main/src/2025/day_07_2.c
fn solution(
    grid: &Vec<Vec<char>>,
    rows: usize,
    columns: usize,
    i: usize,
    j: isize,
    cache: &mut Vec<Vec<Option<u128>>>,
) -> u128 {

    if i >= rows {
        return 1;
    }

    if j < 0 || j as usize >= columns {
        return 1;
    }

    let ju = j as usize;

    if let Some(v) = cache[i][ju] {
        return v;
    }

    let result = if grid[i][ju] != '^' {
        solution(grid, rows, columns, i + 1, j, cache)
    } else {
        solution(grid, rows, columns, i + 1, j - 1, cache)
            + solution(grid, rows, columns, i + 1, j + 1, cache)
    };

    cache[i][ju] = Some(result);
    result
}

fn count_timelines(grid: &Vec<Vec<char>>, rows: usize, columns: usize, start_col: usize) -> u128 {
    let mut cache: Vec<Vec<Option<u128>>> = vec![vec![None; columns]; rows];
    solution(grid, rows, columns, 0, start_col as isize, &mut cache)
}


