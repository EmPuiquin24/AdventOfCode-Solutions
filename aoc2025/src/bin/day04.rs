use std::fs;

fn main() {
    
    let aoc_input = fs::read_to_string("inputs/day04.txt").expect("Document couldn't be read");
    
    let mut grid: Vec<Vec<char>> = aoc_input.lines().map(|line| line.chars().collect()).collect();
    
    let rows = grid.len();
    let cols = grid[0].len();
    
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];
    let mut rolls_accessed = 0;
    let mut total_removed = 0;
    
    let mut it = 0;
    loop {
        let mut to_remove = Vec::new();

        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] != '@' {
                    continue;
                }

                let mut neighbors = 0;

                for (dx, dy) in directions {
                    let ni = i as isize + dx;
                    let nj = j as isize + dy;

                    if ni >= 0 && ni < rows as isize && nj >= 0 && nj < cols as isize {
                        if grid[ni as usize][nj as usize] == '@' {
                            neighbors += 1;
                        }
                    }
                }

                if neighbors < 4 {
                    to_remove.push((i,j));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for (i, j) in &to_remove {
            grid[*i][*j] = '.';
        }

        if it == 0 {
            rolls_accessed += to_remove.len();
            it += 1; 
        }

        total_removed += to_remove.len();
    }

    println!("The number of rolls_accesed (part1) is: {}", rolls_accessed);
    println!("The number of rolls removed (part2) is: {}", total_removed);
}
