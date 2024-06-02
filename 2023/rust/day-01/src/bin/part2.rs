fn main(){
    let input: &str = include_str!("../../input.txt");
    let output: u32 = part2(input);
    dbg!(output);
}


fn part2(input: &str) -> u32 {
    let lines = input.lines();
    let mut new_lines = Vec::new();
    for line in lines {
        let new_line: &str;
        for idx in 0..line.len() {
            let partial_line = &line[idx..];


            if partial_line.starts_with("one"){
                new_line = format!("1{new_line}");
            }
            if partial_line.starts_with("two"){
                line.to_string().insert(idx, '2');
                new_l = ines.push(line);
            }
            if partial_line.starts_with("three"){
                line.to_string().insert(idx, '3');
                new_lines.push(line);
            }
            if partial_line.starts_with("four"){
                line.to_string().insert(idx, '4');
                new_lines.push(line);
            }
            if partial_line.starts_with("five"){
                line.to_string().insert(idx, '5');
                new_lines.push(line);
            }
            if partial_line.starts_with("six"){
                line.to_string().insert(idx, '6');
                new_lines.push(line);
            }
            if partial_line.starts_with("seven"){
                line.to_string().insert(idx, '7');
                new_lines.push(line);
            }
            if partial_line.starts_with("eight"){
                line.to_string().insert(idx, '8');
                new_lines.push(line);
            }
            if partial_line.starts_with("nine"){
                line.to_string().insert(idx, '9');
                new_lines.push(line);
            }

            let latest = new_lines.last();
            dbg!(latest);
        } 
    }

    

    return 200;
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