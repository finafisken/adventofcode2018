use std::collections::{HashMap, LinkedList};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let input_numbers: Vec<u32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    println!("Part 1: {}", part1(input_numbers[0], input_numbers[1]));
    println!(
        "Part 2: {}",
        part2(input_numbers[0], input_numbers[1] * 100)
    );
}

fn part1(players: u32, highest_marble: u32) -> u32 {
    let mut circle: Vec<u32> = vec![0];
    let mut current_marble_idx: u32 = 0;
    let mut player_scores: HashMap<u32, u32> = HashMap::with_capacity(players as usize);
    let mut current_player = 1;
    for marble in 1..=highest_marble {
        if marble % 23 == 0 {
            // score
            let ccw7 = if current_marble_idx >= 7 {
                current_marble_idx - 7
            } else {
                circle.len() as u32 - (7 - current_marble_idx)
            };
            let ccw7_marble = circle.remove(ccw7 as usize);
            *player_scores.entry(current_player).or_insert(0) += marble + ccw7_marble;
            current_marble_idx = ccw7;
        } else {
            // normal insert
            let insert_at = (current_marble_idx as usize + 2) % circle.len();
            circle.insert(insert_at, marble);
            current_marble_idx = insert_at as u32;
        }
        // new current player
        current_player = marble % players;
        if current_player == 0 {
            current_player = players;
        }
    }

    *player_scores.values().max().unwrap()
}

fn part2(players: u32, highest_marble: u32) -> u32 {
    let mut circle: LinkedList<u32> = LinkedList::new();
    circle.push_back(0);
    let mut player_scores: HashMap<u32, u32> = HashMap::with_capacity(players as usize);
    let mut current_player = 1;

    for marble in 1..=highest_marble {
        if marble % 23 == 0 {
            // score
            for _ in 0..7 {
                let step_cww = circle.pop_front().unwrap();
                circle.push_back(step_cww);
            }
            let ccw7_marble = circle.pop_front().unwrap();
            let new_current_marble = circle.pop_back().unwrap();
            circle.push_front(new_current_marble);

            *player_scores.entry(current_player).or_insert(0) += marble + ccw7_marble;
        } else {
            // normal insert
            for _ in 0..1 {
                let step_cw = circle.pop_back().unwrap();
                circle.push_front(step_cw);
            }
            circle.push_front(marble);
        }
        // new current player
        current_player = marble % players;
        if current_player == 0 {
            current_player = players;
        }
    }

    *player_scores.values().max().unwrap()
}

#[test]
fn test_part1() {
    assert_eq!(part1(9, 25), 32);
    assert_eq!(part1(10, 1618), 8317);
    assert_eq!(part1(13, 7999), 146373);
    assert_eq!(part1(17, 1104), 2764);
    assert_eq!(part1(21, 6111), 54718);
    assert_eq!(part1(30, 5807), 37305);
}

#[test]
fn test_part2() {
    assert_eq!(part2(9, 25), 32);
    assert_eq!(part2(10, 1618), 8317);
    assert_eq!(part2(13, 7999), 146373);
    assert_eq!(part2(17, 1104), 2764);
    assert_eq!(part2(21, 6111), 54718);
    assert_eq!(part2(30, 5807), 37305);
}
