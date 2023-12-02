### Setting up a Day in Rust

```bash
// create a new binary
$ cargo new day-01

// confirm it worked
$ cd day-01 && cargo run

// create part1 and part2 files
$ mkdir src/bin && touch src/bin/part1.rs src/bin/part2.rs

// how to run each part
$ cargo run --bin part1
```
any dependencies can be added into the cargo.toml


General files should look like this and tests are added to the bottom of the files they are to run in and can get run using the ```Cargo test``` command

```rust

fn main(){
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}


fn part1(input: &str) -> String {
    "4".to_string()
}


#[cfg(test)]
mod tests {
    // lets you get all funcitons from above
    use super::*;

    #[test]
    fn it_works() {
        let test_input = include_str!("../../test.txt")
        let result = part1("");
        assert_eq!(result, "4".to_string());
    }
}

```
