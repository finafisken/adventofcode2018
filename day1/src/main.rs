use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

// Passing iterators as arguments is hard :(
fn part1(input: &String) -> i32 {
    // Sum of all changes
    input.lines().map(|n| n.parse::<i32>().unwrap()).sum()
}

fn part2(input: &String) -> i32 {
    // First (partial) sum to accour twice
    let mut prev_sums = HashMap::new();
    let mut cur_sum = 0;

    for line in input.lines().cycle() {
        prev_sums.insert(cur_sum, true);
        let delta: i32 = line.parse().unwrap();
        cur_sum += delta;
        if prev_sums.contains_key(&cur_sum) {
            break;
        }
    }
    cur_sum
}

#[test]
fn validate_part1() {
    assert_eq!(3, part1(&"+1\n+1\n+1".to_string()));
    assert_eq!(0, part1(&"+1\n+1\n-2".to_string()));
    assert_eq!(-6, part1(&"-1\n-2\n-3".to_string()));
}

#[test]
fn validate_part2() {
    assert_eq!(0, part2(&"+1\n-1".to_string()));
    assert_eq!(10, part2(&"+3\n+3\n+4\n-2\n-4".to_string()));
    assert_eq!(5, part2(&"-6\n+3\n+8\n+5\n-6".to_string()));
    assert_eq!(14, part2(&"+7\n+7\n-2\n-7\n-4".to_string()));
}
