fn main(){
    let input: &str = include_str!("../../input.txt");
    let output: u32 = part2(input);
    dbg!(output);
}


fn part2(input: &str) -> u32 {
    let output: u32 = input.lines()        
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
        let test_input: &str = include_str!("../../test2.txt");
        let result: u32 = part2(test_input);
        assert_eq!(result, 281);
    }
}