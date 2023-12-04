use std::collections::HashSet;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, digit1, space0},
    multi::fold_many1, 
    sequence::{delimited, separated_pair, tuple, terminated},
};

fn main(){
    let input: &str = include_str!("../../input.txt");
    let output: u32 = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let output: u32 = input.lines()
                    .map(|line: &str| {
                        match parse_game(line) {
                            Ok((_, game)) => {
                                dbg!(game.score());
                                return game.score();                                
                            }
                            Err(err) => {
                                println!("Error parsing line: {:?}", err);
                                0 // Assuming 2^0 in case of parsing error
                            }
                        }
                    })
                    .sum(); // Sum the results
            
                return output
            }

#[derive(Debug)]
struct Game{
    winning_numbers: HashSet<u32>,
    playing_numbers: HashSet<u32>
}

impl Game {
    // Method to find the number of overlaps between winning_numbers and numbers
    fn score(&self) -> u32 {
        let overlap_count = self
            .winning_numbers
            .intersection(&self.playing_numbers)
            .count() as u32;

        if overlap_count == 0 {
            return 0
        } else {
            return 2_u32.pow(overlap_count-1) as u32;
        }

        
    }
}

fn set(input: &str) -> IResult<&str, HashSet<u32>> {
    fold_many1(
        terminated(complete::u32, space0),
        HashSet::new,
        |mut acc: HashSet<_>, item| {
            acc.insert(item);
            acc
        },
    )(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = delimited(
        tuple((tag("Card"), space0)),
        digit1,
        tuple((tag(":"), space0)),
    )(input)?;
    separated_pair(set, tuple((tag("|"), space0)), set)
        .map(|(winning_numbers, playing_numbers)| Game {
            winning_numbers,
            playing_numbers,
        })
        .parse(input)
}


#[cfg(test)]
mod tests {
    // lets you get all funcitons from above
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]

    fn line_test(
        #[case] line: &str,
        #[case] expected: u32,
    ) {
        let (_, card) = parse_game(line).expect("should be a valid card");
        assert_eq!(expected, card.score())
    }

    #[test]
    fn it_works() {
        let test_input: &str = include_str!("../../test.txt");
        let result = part1(test_input);
        assert_eq!(result, 13);
    }
}
