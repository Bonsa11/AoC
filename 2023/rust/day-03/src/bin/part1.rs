use std::collections::HashSet;

fn main(){
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}


fn part1(input: &str) -> i32 {
    
    let lines = input.lines();
    let set_index = get_symbol_indexes(input);

    for (idy, row) in lines.enumerate(){
        for (idx, character) in row.chars().enumerate() {

        }
    }
    for index in set_index.iter() {
        for x in -1_i32..=1 {
            for y in -1_i32..=1 {
            
            // Convert indices to i32 before using them in the calculation
            let new_idx = (idx as i32 + x) as usize;
            let new_idy = (idy as i32 + y) as usize;

            // check if grid around contains number
            if new_idx == 0 && new_idy ==0 && new_idx < lines.len() && new_idy < row.len() && lines[new_idx][new_idy].is_digit(10) {
                
                
            let found_value = lines[new_idx][new_idy];
            //println!("\t {found_value}: {new_idx},{new_idy}");
    }

                
                        



                        }
                    }
                }

                todo!();
            } 

fn get_symbol_indexes(input: &str) -> HashSet<(usize, usize)> {
    let lines: Vec<Vec<char>> = input.lines().map(|line| {line.chars().collect()}).collect();
    let mut set_indexes = HashSet::new();

    // iterate over rows
    for (idy, row) in lines.iter().enumerate() {
        
        // iterate over columns
        for (idx, character) in row.iter().enumerate() {
            
            // define condititnal for character
            let is_valid_symbol = !character.is_digit(10) && *character != '.';
            
            // look for numbers in grid around the symbol
            if is_valid_symbol {

                // add symbol indexes to set
                set_indexes.insert((idx,idy));
            }
        }
    }

    return set_indexes;
}

fn get_numbers(input: &str) {
    let lines = input.lines();
    let mut num_indexes = HashSet::new();

    for (idx, row) in lines.enumerate() {
        for (idy, character) in row.chars().enumerate() {
            if character.is_digit(10) {
                
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // lets you get all funcitons from above
    use super::*;

    #[test]
    fn it_works() {
        let test_input: &str = include_str!("../../test.txt");
        let result = part1(test_input);
        assert_eq!(result, 4361);
    }
}
