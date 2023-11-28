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

fn main() {
    println!("{:?}", generate_grid(57).get(&(122, 79)));
    println!("{:?}", generate_grid(39).get(&(217, 196)));
    println!("{:?}", generate_grid(71).get(&(101, 153)));
}
