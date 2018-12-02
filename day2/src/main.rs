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
            println!("{:?} present {:?} times", u_letter, count);
        }
    }
    twos * threes
}

fn part2(input: &String) -> i32 {
    2
}

#[test]
fn validate_part1() {
    let input = "abcdef\n
        bababc\n
        abbcde\n
        abcccd\n
        aabcdd\n
        abcdee\n
        ababab".to_string();

    assert_eq!(12, part1(&input));
}

// #[test]
// fn validate_part2() {
//     assert_eq!(0, part2(&"+1\n-1".to_string()));
// }
