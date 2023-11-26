use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    velocity_x: i32,
    velocity_y: i32,
    origin_x: i32,
    origin_y: i32,
}

impl Point {
    fn new(num: Vec<i32>) -> Self {
        Self {
            x: num[0],
            y: num[1],
            velocity_x: num[2],
            velocity_y: num[3],
            origin_x: num[0],
            origin_y: num[1],
        }
    }

    // fn pos_after(&self, steps: usize) -> Point {
    //     Point {
    //         x: self.origin_x + (self.velocity_x * steps as i32),
    //         y: self.origin_y + (self.velocity_y * steps as i32),
    //         ..*self
    //     }
    // }

    fn pos_after(&mut self, steps: usize) -> &Self {
        self.x = self.origin_x + (self.velocity_x * steps as i32);
        self.y = self.origin_y + (self.velocity_y * steps as i32);
        self
    }
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let regex = Regex::new(r"-?\d+").unwrap();

    // let test = input.lines().collect::<Vec<&str>>();
    for line in input.lines() {
        let numbers: Vec<i32> = regex
            .find_iter(line)
            .filter_map(|s| s.as_str().parse().ok())
            .collect();
        let point = Point::new(numbers);
        println!("{:?}", point);
    }
    // println!("{:?}", test);
    // let input_numbers: Vec<u32> = input
    // .split_whitespace()
    // .filter_map(|s| s.parse().ok())
    // .collect();
}
