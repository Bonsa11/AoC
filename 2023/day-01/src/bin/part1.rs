fn main(){
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}

// works but makes part2 super difficult
fn original_part1(input: &str) -> i32 {
    // splittig input by line
    let lines = input.split("\n");

    // creating an empty array
    let mut values: Vec<i32> = vec![];
    
    // iterating over lines
    for line in lines{

        // using double pointers to traverse line for digits

        let characters: Vec<char> = line.chars().collect();
        let mut lp= 0;
        let mut rp= characters.len() -1;


        while !characters[lp].is_digit(10) {
            lp += 1
        };

        while !characters[rp].is_digit(10){
            rp -= 1
        };

        let line_value: i32 = (characters[lp].to_string() + &characters[rp].to_string()).parse::<i32>().unwrap();

        values.push(line_value);
        println!("{line_value}")
    }

    return values.iter().sum(); 


}

// borrowed from https://github.com/ChristopherBiscardi/advent-of-code/tree/76c5ca80795336e465c1272d99147a069162de56/2023/rust/day-01
fn part1(input: &str) -> u32 {
    let output = input.lines()        
                    .map(|line| {
                        let mut it = line
                                        .chars()
                                        .filter_map(|character| {
                                            character.to_digit(10)
                                        });
                        let first = it.next().expect("Should be a number");

                        match it.last() {
                            Some(num) => format!("{first}{num}"),
                            None => format!("{first}{first}"),
                        }.parse::<u32>()
                        .expect("Should be a valid number")
                    }).sum::<u32>();
    return output
}



#[cfg(test)]
mod tests {
    // lets you get all funcitons from above
    use super::*;

    #[test]
    fn it_works() {
        let test_input = include_str!("../../test.txt");
        let result = part1(test_input);
        assert_eq!(result, 142);
    }
}