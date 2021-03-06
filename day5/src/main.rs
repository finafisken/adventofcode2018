use std::fs::File;
use std::io::prelude::*;
use std::cmp::min;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

// fold over string
// for each fold check if last char of acc reacts with next
// if they react remove last off acc and ignore next, if not add next to acc
// count number of chars

fn part1(input: &String) -> i32 {
    let reduced_polymer = input.chars().fold(" ".to_string(), |mut acc, c| {
        let last_char = acc.chars().last().unwrap();
        if last_char.to_ascii_lowercase() == c.to_ascii_lowercase() && (last_char.is_lowercase() && c.is_uppercase() || last_char.is_uppercase() && c.is_lowercase()) {
            acc.chars().take(acc.chars().count() - 1).collect()
        } else {
            acc.push(c);
            acc
        }
    });

    let reduced_polymer = reduced_polymer.trim_start();

    reduced_polymer.chars().count() as i32
}

fn part2(input: &String) -> i32 {
    let mut min_count = 11894; // result from part 1

    // a-z range in bytes
    for c in b'a'..b'z' {
        let alpha = c as char;
        let trimmed_polymer: String = input.chars().filter(|letter| letter.to_ascii_lowercase() != alpha).collect();
        min_count = min(min_count, part1(&trimmed_polymer));
    }
    min_count
}

#[test]
fn validate_part1() {
    let input = "dabAcCaCBAcCcaDA".to_string();

    assert_eq!(10, part1(&input));
}

#[test]
fn validate_part2() {
    let input = "dabAcCaCBAcCcaDA".to_string();

    assert_eq!(4, part2(&input));
}
