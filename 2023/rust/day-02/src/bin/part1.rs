fn main(){
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}


fn part1(input: &str) -> u32 {
    let lines = input.lines();

    let mut total = 0;

    // get id and sums of cube for each game
    for game_line in lines {
        let id = get_id(game_line);
        let passes_check = sum_cubes(game_line);

        if passes_check {
            dbg!(id);
            total += id;
        }
        
    }        
                        
    return total;
}


fn get_id(game_line: &str) -> u32 {
    let start_index: u32 = 5;
    let end_index = game_line.find(":").expect("Line will always contains a :");
    
    let start_index_usize = start_index as usize;
    let end_index_usize = end_index as usize;
    
    let id_str = &game_line[start_index_usize..end_index_usize];
    let id: u32 = id_str.parse::<u32>().unwrap();

    return id
}


fn sum_cubes(game_line: &str) -> bool{

    // set some standard vars
    let r_max = 12; 
    let g_max = 13;
    let b_max = 14;

    let start_cubes = game_line.find(":").expect("Should contain a :") + 1;
    let start_cubes_usize = start_cubes as usize;
    let game_string = &game_line[start_cubes_usize..];

    let turns = game_string.split(";");
    for turn in turns{

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        

        //dbg!(turn);
        let cubes = turn.trim().split(",");
        for cube in cubes {
            let cube = cube.trim();
            //dbg!(cube);
            if cube.contains("red"){
                let end_index = cube.find(" ").expect("Always will contain a space");
                //dbg!(&cube.trim()[0..end_index]);
                let value = &cube.trim()[0..end_index].parse::<u32>().unwrap();

                red += value
            };

            if cube.contains("green") {
                let end_index = cube.find(" ").expect("Always will contain a space");
                //dbg!(&cube.trim()[0..end_index]);
                let value = &cube.trim()[0..end_index].parse::<u32>().unwrap();

                green += value
            };

            if cube.contains("blue") {
                let end_index = cube.find(" ").expect("Always will contain a space");
                //dbg!(&cube.trim()[0..end_index]);
                let value = &cube.trim()[0..end_index].parse::<u32>().unwrap();

                blue += value
            };

        }

        // add id to 
        let conditions  = red <= r_max && green <= g_max && blue <= b_max;
        if !conditions {
            return false
        }

    }

    return true
}


#[cfg(test)]
mod tests {
    // lets you get all funcitons from above
    use super::*;

    #[test]
    fn it_works() {
        let test_input: &str = include_str!("../../test.txt");
        let result: u32 = part1(test_input);
        assert_eq!(result, 8);
    }
}