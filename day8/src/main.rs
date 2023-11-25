use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    println!("Part 1: {}", part1(&input));
    // println!("Part 2: {}", part2(&input));
}

fn sum_meta(nodes: &mut impl Iterator<Item = u32>) -> u32 {
    let child_nodes = nodes.next().unwrap();
    let meta_entries = nodes.next().unwrap() as usize;

    let c_sum: u32 = (0..child_nodes).map(|_| sum_meta(nodes)).sum();
    let m_sum: u32 = nodes.take(meta_entries).sum();

    c_sum + m_sum
}

fn part1(input: &str) -> u32 {
    let mut nodes = input.split_whitespace().map(|n| n.parse::<u32>().unwrap());

    sum_meta(&mut nodes)
}

// fn part2(input: &String) -> u32 {
//     2
// }

#[test]
fn validate_part1() {
    let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2".to_string();

    assert_eq!(138, part1(&input));
}

// #[test]
// fn validate_part2() {
//     let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2".to_string();

//     assert_eq!(66, part2(&input));
// }
