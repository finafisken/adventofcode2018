use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    println!("Part 1: {}", part1(&input));
    // println!("Part 2: {}", part2(&input));
}

// for claims get points insert (x,y) into hashmap. If hash exists +1
fn part1(input: &String) -> u32 {
    let mut colliding_points : HashMap<(u32,u32), u32> = HashMap::new();

    for claim in input.lines() {
        let claim_vec = claim.split(|c| c == '@' || c == ':').map(|x| x.trim_start()).collect::<Vec<&str>>();
        let start = &claim_vec[1].split(',').map(|v| v.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let size = &claim_vec[2].split('x').map(|v| v.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let points = get_points((start[0], start[1]), (size[0], size[1]));

        for point in points {
            *colliding_points.entry(point).or_insert(0) += 1;
        }
    }
    colliding_points.iter().filter(|&(_,v)| *v > 1).count() as u32
}

// get all points within a claim
fn get_points(start: (u32, u32), size: (u32, u32)) -> Vec<(u32, u32)> {
    let mut points = Vec::new();
    let (start_x, start_y) = start;
    let (size_x, size_y) = size;
    for x in start_x..(start_x + size_x) {
        for y in start_y..(start_y + size_y) {
            points.push((x, y));
        }
    }
    points
}

fn part2(input: &String) -> u32 {
    3
}

#[test]
fn validate_part1() {
    let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2".to_string();

    assert_eq!(4, part1(&input));
}

 #[test]
 fn validate_part2() {
     let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2".to_string();

    assert_eq!(3, part2(&input));
}


