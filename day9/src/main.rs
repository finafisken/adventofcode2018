use std::collections::HashMap;
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
}

fn part1(players: u32, highest_marble: u32) -> u32 {
    let mut circle: Vec<u32> = vec![0];
    let mut current_marble_idx: u32 = 0;
    let mut player_scores: HashMap<u32, u32> = HashMap::with_capacity(players as usize);
    let mut current_player = 1;
    for marble in 1..=highest_marble {
        if marble % 23 == 0 {
            // score
            let mut ccw7: i32 = current_marble_idx as i32 - 7;
            if ccw7 < 0 {
                ccw7 = circle.len() as i32 - ccw7;
            }
            let ccw7_marble = circle.remove(ccw7 as usize);
            *player_scores.entry(current_player).or_insert(0) += marble + ccw7_marble;
            current_marble_idx = ccw7 as u32;
        } else {
            // normal insert
            if current_marble_idx == 0 {
                circle.push(marble);
                current_marble_idx = 1;
            } else {
                let insert_at = (current_marble_idx as usize + 2) % circle.len();
                circle.insert(insert_at, marble);
                current_marble_idx = insert_at as u32;
            }
        }
        // new current player
        current_player = marble % players + 1;
        println!("{:?}", circle);
        println!("{:?}", circle[current_marble_idx as usize]);
    }

    println!("## {:?}", player_scores);
    // println!("{:?}", circle);
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
