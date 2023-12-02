fn main(){
    let input = include_str!("../../input.txt");
    let output = part2(input);
    dbg!(output);
}


fn part2(input: &str) -> i32 {
    let elves = input.split("\n\n");
    let mut calories: Vec<i32> = Vec::new();
    for elf in elves {
        let meals = elf.split("\n");
        let mut cals: i32 = 0;
        for meal in meals {
            cals += meal.parse::<i32>().unwrap();
        }
        calories.push(cals);
    }
    calories.sort();

    let sum: i32 = calories.iter().rev().take(3).sum();            
    return sum;
} 


#[cfg(test)]
mod tests {
    // lets you get all funcitons from above
    use super::*;

    #[test]
    fn it_works() {
        let test_input: &str = include_str!("../../test.txt");
        let result = part2(test_input);
        assert_eq!(result, 45000);
    }
}
