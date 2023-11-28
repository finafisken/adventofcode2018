use std::collections::HashMap;

type Grid = HashMap<(usize, usize), i32>;

fn hundredth_digit(number: i32) -> i32 {
    let n_string = format!("{:0>#3}", number.to_string());
    n_string
        .chars()
        .rev()
        .nth(2)
        .expect("less than 3 numbers")
        .to_digit(10)
        .expect("failed to convert to digit") as i32
}

fn generate_grid(serial_nr: usize) -> Grid {
    let mut grid: Grid = HashMap::new();

    for y in 1..=300 {
        for x in 1..=300 {
            let rack_id = x + 10;
            let num = (rack_id * y + serial_nr) * rack_id;

            let power_level: i32 = hundredth_digit(num as i32) - 5;
            grid.insert((x, y), power_level);
        }
    }

    grid
}

fn part1(serial_nr: usize) -> (usize, usize) {
    let grid = generate_grid(serial_nr);
    let mut max_3x3 = (0, 0, 0);
    for y in 1..=297 {
        for x in 1..=297 {
            let mut to_sum = Vec::new();
            for yy in y..=(y + 2) {
                for xx in x..=(x + 2) {
                    to_sum.push(grid[&(xx, yy)]);
                }
            }
            let sum = to_sum.iter().sum();
            if sum > max_3x3.0 {
                max_3x3 = (sum, x, y);
            }
        }
    }

    (max_3x3.1, max_3x3.2)
}

fn main() {
    println!("Part 1: {:?}", part1(9798));
}
