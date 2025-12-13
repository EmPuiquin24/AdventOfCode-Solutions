use std::fs;
use regex::Regex;

fn main() {
    let aoc_input = fs::read_to_string("inputs/day12.txt").expect("Document couldn't be read");
    let parts: Vec<&str> = aoc_input.split("\n\n").collect();
    let re = Regex::new(r"\d+").unwrap();

    let raw_shapes = &parts[..parts.len() - 1];
    let raw_regions = parts.last().unwrap();

    const SHAPE_HEIGHT: usize = 3;
    const SHAPE_WIDTH: usize = 3;
    
    let mut shape_num_tiles: Vec<usize> = Vec::new();

    for raw_shape in raw_shapes {
        let mut lines = raw_shape.lines();

        let _id = lines.next().unwrap();
        let grid: Vec<&str> = lines.collect();
    
        assert_eq!(grid.len(), SHAPE_HEIGHT);
        assert!(grid.iter().all(|row| row.len() == SHAPE_WIDTH));

        let count = grid.iter().flat_map(|row| row.chars()).filter(|&ch| ch == '#').count();

        shape_num_tiles.push(count);
    }
    
    let mut total = 0;
    
    for raw_region in raw_regions.lines() {
        let nums: Vec<usize> = re.find_iter(raw_region).map(|m| m.as_str().parse().unwrap()).collect();

        let width = nums[0];
        let height = nums[1];
        let shape_quantities = &nums[2..];

        let max_presents_lower_bound = (width / SHAPE_WIDTH) * (height / SHAPE_HEIGHT);
        let num_presents: usize = shape_quantities.iter().sum();

        if num_presents <= max_presents_lower_bound {
            total += 1;
            continue;
        }

        let num_tiles_lower_bound: usize = shape_quantities.iter().enumerate().map(|(i, &qty)| qty * shape_num_tiles[i]).sum();

        let region_num_tiles = width * height;

        if num_tiles_lower_bound > region_num_tiles {
            continue;
        }
        
        panic!("Baje de chisito");
    }

    println!("Regions that can fit all the presents: {}", total);

}

