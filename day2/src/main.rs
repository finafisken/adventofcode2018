use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &String) -> u32 {
    let mut twos = 0;
    let mut threes = 0;
    for line in input.lines() {
        let mut unique = HashSet::new();
        let mut seen_two = false;
        let mut seen_three = false;
        for letter in line.chars() {
            unique.insert(letter);
        }
        for u_letter in unique {
            let count = line.matches(u_letter).collect::<Vec<&str>>().len();
            match count {
                2 => {
                    if !seen_two {
                        twos += 1;
                        seen_two = true;
                    }
                    
                },
                3 => {
                    if !seen_three {
                        threes += 1;
                        seen_three = true;
                    }
                },
                _ => {}
            }
            if seen_three && seen_two {
                break;
            }
        }
    }
    twos * threes
}

fn part2(input: &String) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let str_length = lines.first().unwrap().chars().count();
    for idx in 1..str_length {
        let mut strings_without_idx_char = HashSet::new();
        for line in &lines {
            let val: String = line.chars().take(idx-1).chain(line.chars().skip(idx)).collect();
            if !strings_without_idx_char.insert(val.clone()) {
                return val
            }
        }
    }
    "".to_string()
}

#[test]
fn validate_part1() {
    let input = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab".to_string();

    assert_eq!(12, part1(&input));
}

 #[test]
 fn validate_part2() {
    let input = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz".to_string();

    assert_eq!("fgij", part2(&input));
}
