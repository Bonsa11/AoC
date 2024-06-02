fn main(){
    let input = include_str!("../../input.txt");
    let output = part2(input);
    dbg!(output);
}


fn part2(input: &str) -> u32 {
    let lines = input.lines();

    let mut total = 0;

    // get id and sums of cube for each game
    for game_line in lines {
        let val = product_cubes(game_line);
        dbg!(val);
        total += val;
    };
    
    return total;
}        


fn product_cubes(game_line: &str) -> u32 {

    // set some standard vars
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    // parse text
    let start_cubes = game_line.find(":").expect("Should contain a :") + 1;
    let start_cubes_usize = start_cubes as usize;
    let game_string = &game_line[start_cubes_usize..];

    let turns = game_string.split(";");
    for turn in turns{
        //dbg!(turn);
        let cubes = turn.trim().split(",");
        for cube in cubes {
            let cube = cube.trim();
            //dbg!(cube);
            if cube.contains("red"){
                let end_index = cube.find(" ").expect("Always will contain a space");
                //dbg!(&cube.trim()[0..end_index]);
                let value = &cube.trim()[0..end_index].parse::<u32>().unwrap();

                if value > &red {
                    red = *value
                }
            };

            if cube.contains("green") {
                let end_index = cube.find(" ").expect("Always will contain a space");
                //dbg!(&cube.trim()[0..end_index]);
                let value = &cube.trim()[0..end_index].parse::<u32>().unwrap();

                if value > &green {
                    green = *value
                }
            };

            if cube.contains("blue") {
                let end_index = cube.find(" ").expect("Always will contain a space");
                //dbg!(&cube.trim()[0..end_index]);
                let value = &cube.trim()[0..end_index].parse::<u32>().unwrap();

                if value > &blue {
                    blue = *value
                }
            };

        };
    }

    return red*green*blue
}


#[cfg(test)]
mod tests {
    // lets you get all funcitons from above
    use super::*;

    #[test]
    fn it_works() {
        let test_input: &str = include_str!("../../test.txt");
        let result: u32 = part2(test_input);
        assert_eq!(result, 2286);
    }
}