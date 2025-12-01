use std::fs;

fn main() {

    let aoc_input = fs::read_to_string("inputs/day01.txt").expect("Document couldn't be read");
    let rows: Vec<&str> = aoc_input.lines().collect();

    let mut f_num = 50;
    let mut z_counter = 0;
    let mut z_p_counter = 0; 
    
    for row in &rows {

        let (r_type, str_num) = row.split_at(1);
        let r_num: i32 = str_num.parse().unwrap();
        
        if r_type == "R" {

            z_p_counter += (f_num + r_num) / 100;
            f_num = (f_num + r_num) % 100;

        } else {

            if f_num == 0 {
                z_p_counter += r_num / 100;
            } else if r_num >= f_num {
                z_p_counter += (r_num - f_num) / 100 + 1;
            }
            f_num = ((f_num - r_num % 100) + 100) % 100;

        }
        
        if f_num == 0 {
            z_counter += 1;
        }
    }

    println!("Password 1: {}", z_counter);
    println!("Password 2: {}", z_p_counter);
}
