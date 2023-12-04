fn main(){
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}


fn part1(input: &str) -> i32 {
    let mut max_calories: i32 = 0;
    let mut current_calories: i32 = 0;
    let lines = input.split('\n');
    for line in lines {
        if line.is_empty() {
            if current_calories > max_calories {
                max_calories = current_calories;
            } current_calories = 0;
        } else {
            let additional_calories = line.parse::<i32>().unwrap();
            current_calories += additional_calories;
        }

    }
    return max_calories;
} 


#[cfg(test)]
mod tests {
    // lets you get all funcitons from above
    use super::*;

    #[test]
    fn it_works() {
        let test_input: &str = include_str!("../../test.txt");
        let result = part1(test_input);
        assert_eq!(result, 24000);
    }
}
