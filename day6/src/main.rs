use std::fs::File;
use std::io::prelude::*;
use std::cmp;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

// figure out board size (max/min x/y tot)
// iterate over all points in board, calc manhattan distance to each point
// assign point to closest coord

fn part1(input: &String) -> i32 {
    let coordinates: Vec<(i32, i32)> = input.lines().map(|p| {
        let p_vec: Vec<&str> = p.split(", ").collect();
        (p_vec[0].parse().unwrap(), p_vec[1].parse().unwrap())
    }).collect();

    let (max_x, max_y) = coordinates.iter().fold((0, 0), |(max_x, max_y), (x, y)| (cmp::max(max_x, *x), cmp::max(max_y, *y)));
    let (min_x, min_y) = coordinates.iter().fold((400, 400), |(min_x, min_y), (x, y)| (cmp::min(min_x, *x), cmp::min(min_y, *y)));

    let mut coord_areas = vec![0; coordinates.len()];
    for x in min_x..max_x {
        for y in min_y..max_y {
            // owned by point {index} and closest distance
            // 1337 is used to indicate a point is not owned by any coord
            let (owned_by, _) = coordinates.iter().enumerate().fold((1337, max_x + max_y), |(owner, distance), (i, (cx, cy))| {
               let c_dist = (x - cx).abs() + (y - cy).abs();
               if c_dist == distance {
                   return (1337, distance)
               }
               if c_dist < distance {
                   return (i, c_dist)
               }
               return (owner, distance)
            });
            if owned_by != 1337 {
                coord_areas[owned_by] += 1;
            }
        }
    }

    *coord_areas.iter().max().unwrap()
}

fn part2(input: &String) -> i32 {
    let coordinates: Vec<(i32, i32)> = input.lines().map(|p| {
        let p_vec: Vec<&str> = p.split(", ").collect();
        (p_vec[0].parse().unwrap(), p_vec[1].parse().unwrap())
    }).collect();

    let (max_x, max_y) = coordinates.iter().fold((0, 0), |(max_x, max_y), (x, y)| (cmp::max(max_x, *x), cmp::max(max_y, *y)));
    let (min_x, min_y) = coordinates.iter().fold((400, 400), |(min_x, min_y), (x, y)| (cmp::min(min_x, *x), cmp::min(min_y, *y)));

    let mut region_area = 0;
    for x in min_x..max_x {
        for y in min_y..max_y {
            let threshold = 10000; // 32 for test
            let distance_sum: i32 = coordinates.iter().map(|(cx, cy)| (x - cx).abs() + (y - cy).abs()).sum();
            if distance_sum < threshold {
                region_area += 1;
            }
        }
    }

    region_area
}

#[test]
fn validate_part1() {
    let input = "1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9".to_string();

    assert_eq!(17, part1(&input));
}

#[test]
fn validate_part2() {
    let input = "1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9".to_string();

    assert_eq!(16, part2(&input));
}
