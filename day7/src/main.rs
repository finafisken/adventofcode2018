extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeSet;
use regex::Regex;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    println!("Part 1: {}", part1(&input));
    // println!("Part 2: {}", part2(&input));
}

fn part1(input: &String) -> String {
    let mut dependencies: HashMap<char, Vec<char>> = HashMap::new();
    let regx = Regex::new(r"([A-Z]).+([A-Z]).+([A-Z]).+").unwrap(); // fix regex
    for line in input.lines() {
        let caps = regx.captures(line).unwrap();
        let req = caps[2].chars().next().unwrap();
        let target = caps[3].chars().next().unwrap();
        dependencies.entry(target).or_default().push(req);
        dependencies.entry(req).or_default();
    }

    let mut remaining = dependencies.keys().cloned().collect::<BTreeSet<_>>();
    let mut fulfilled = HashSet::new();
    let mut ordered = Vec::new();

    while !remaining.is_empty() {
        for c in remaining.iter().cloned() {
            let ready = match dependencies.get(&c) {
                Some(dependencies) => dependencies.iter().all(|dep| fulfilled.contains(dep)),
                None => true,
            };

            if ready {
                ordered.push(c);
                fulfilled.insert(c);
                remaining.remove(&c);
                break;
            }
        }
    }

    ordered.into_iter().collect::<String>()
}

// fn part2(input: &String) -> i32 {
//     2
// }

#[test]
fn validate_part1() {
    let input = "Step C must be finished before step A can begin.\nStep C must be finished before step F can begin.\nStep A must be finished before step B can begin.\nStep A must be finished before step D can begin.\nStep B must be finished before step E can begin.\nStep D must be finished before step E can begin.\nStep F must be finished before step E can begin.".to_string();

    assert_eq!("CABDFE", part1(&input));
}

// #[test]
// fn validate_part2() {
//     let input = "Step C must be finished before step A can begin.\nStep C must be finished before step F can begin.\nStep A must be finished before step B can begin.\nStep A must be finished before step D can begin.\nStep B must be finished before step E can begin.\nStep D must be finished before step E can begin.\nStep F must be finished before step E can begin.".to_string();

//     assert_eq!(16, part2(&input));
// }
