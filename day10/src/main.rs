use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    velocity_x: i32,
    velocity_y: i32,
}

impl Point {
    fn new(num: Vec<i32>) -> Self {
        Self {
            x: num[0],
            y: num[1],
            velocity_x: num[2],
            velocity_y: num[3],
        }
    }

    fn tick(&mut self) {
        self.x += self.velocity_x;
        self.y += self.velocity_y;
    }
}

fn paint(points: Vec<Point>, max_x: i32, min_x: i32, max_y: i32, min_y: i32) {
    for y in min_y..=max_y {
        let mut row: Vec<char> = Vec::new();
        for x in min_x..=max_x {
            if points.iter().any(|p| p.x == x && p.y == y) {
                row.push('#');
            } else {
                row.push(' ');
            }
        }
        let output_row: String = row.iter().cloned().collect();
        println!("{output_row}");
    }
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let regex = Regex::new(r"-?\d+").unwrap();

    let mut points = Vec::new();
    for line in input.lines() {
        let numbers: Vec<i32> = regex
            .find_iter(line)
            .filter_map(|s| s.as_str().parse().ok())
            .collect();

        points.push(Point::new(numbers));
    }

    let mut waited_seconds = 0;
    loop {
        waited_seconds += 1;
        for point in &mut points {
            point.tick();
        }

        let max_x = points.iter().map(|p| p.x).reduce(i32::max).unwrap();
        let min_x = points.iter().map(|p| p.x).reduce(i32::min).unwrap();
        let max_y = points.iter().map(|p| p.y).reduce(i32::max).unwrap();
        let min_y = points.iter().map(|p| p.y).reduce(i32::min).unwrap();

        if max_x.abs_diff(min_x) <= 70 && max_y.abs_diff(min_y) <= 30 {
            println!("Part 1:");
            paint(points, max_x, min_x, max_y, min_y);
            break;
        }
    }
    println!("Part 2: {}", waited_seconds);
}
