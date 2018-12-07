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
    println!("Part 2: {}", part2(&input, 5, 60));
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

fn part2(input: &String, worker_count: usize, base_time: u32) -> u32 {
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
    let mut done = Vec::new();

    let mut workers = std::iter::repeat(()).map(|_| Worker {
        work: 0,
        current: None,
    }).take(worker_count).collect::<Vec<_>>();

    let mut tick = 0;

    loop {
        tick += 1;

        let mut idle = Vec::new();

        for worker in &mut workers {
            worker.tick();

            if worker.work == 0 {
                if let Some(c) = worker.current.take() {
                    done.push(c);
                    fulfilled.insert(c);
                }

                idle.push(worker);
            }
        }

        if remaining.is_empty() && idle.len() == worker_count {
            break;
        }

        if idle.is_empty() {
            continue;
        }

        for c in remaining.iter().cloned().collect::<Vec<_>>() {
            if idle.is_empty() {
                break;
            }

            let ok = match dependencies.get(&c) {
                Some(dependencies) => dependencies.iter().all(|dep| fulfilled.contains(&dep)),
                None => true,
            };

            if ok {
                if let Some(w) = idle.pop() {
                    w.work = base_time + (c as u32) - ('A' as u32) + 1;
                    w.current = Some(c);
                    remaining.remove(&c);
                }
            }
        }
    }

    return tick - 1;

    #[derive(Debug)]
    struct Worker {
        // amount left
        work: u32,
        // currently working on
        current: Option<char>,
    }

    impl Worker {
        fn tick(&mut self) {
            if self.work > 0 {
                self.work -= 1;
            }
        }
    }
}

#[test]
fn validate_part1() {
    let input = "Step C must be finished before step A can begin.\nStep C must be finished before step F can begin.\nStep A must be finished before step B can begin.\nStep A must be finished before step D can begin.\nStep B must be finished before step E can begin.\nStep D must be finished before step E can begin.\nStep F must be finished before step E can begin.".to_string();

    assert_eq!("CABDFE", part1(&input));
}

#[test]
fn validate_part2() {
    let input = "Step C must be finished before step A can begin.\nStep C must be finished before step F can begin.\nStep A must be finished before step B can begin.\nStep A must be finished before step D can begin.\nStep B must be finished before step E can begin.\nStep D must be finished before step E can begin.\nStep F must be finished before step E can begin.".to_string();

    assert_eq!(15, part2(&input, 2, 0));
}
