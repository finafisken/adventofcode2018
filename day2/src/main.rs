use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    println!("Part 1: {}", part1(&input));
    // println!("Part 2: {}", part2(&input));
}

fn part1(input: &String) -> u32 {
    1
}

// fn part2(input: &String) -> i32 {
//     2
// }

#[test]
fn validate_part1() {
    let input = "abcdef\n
        bababc\n
        abbcde\n
        abcccd\n
        aabcdd\n
        abcdee\n
        ababab\n".to_string();

    assert_eq!(12, part1(&input));
}

// #[test]
// fn validate_part2() {
//     assert_eq!(0, part2(&"+1\n-1".to_string()));
// }
