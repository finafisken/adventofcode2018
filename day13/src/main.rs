use std::{collections::HashMap, fs, task::Wake};

#[derive(Debug)]
struct Cart {
    position: (usize, usize),
    char: char,
    turns_made: usize,
}

impl Cart {
    fn intersect(&mut self) {
        match self.char {
            '<' => match self.turns_made % 3 {
                1 => self.char = 'v',
                2 => self.char = '<',
                0 => self.char = '^',
                _ => unreachable!(),
            },
            '^' => match self.turns_made % 3 {
                1 => self.char = '<',
                2 => self.char = '^',
                0 => self.char = '>',
                _ => unreachable!(),
            },
            '>' => match self.turns_made % 3 {
                1 => self.char = '^',
                2 => self.char = '>',
                0 => self.char = 'v',
                _ => unreachable!(),
            },
            'v' => match self.turns_made % 3 {
                1 => self.char = '>',
                2 => self.char = 'v',
                0 => self.char = '<',
                _ => unreachable!(),
            },
            _ => panic!("unknown cart char"),
        }
        self.turns_made += 1;
    }

    fn follow(&mut self) {
        let (x, y) = self.position;
        match self.char {
            '<' => self.position = (x - 1, y),
            '^' => self.position = (x, y - 1),
            '>' => self.position = (x + 1, y),
            'v' => self.position = (x, y + 1),
            _ => panic!("unknown cart char"),
        }
    }
}

fn main() {
    let input = fs::read_to_string("test_input.txt").expect("missing input");

    let mut track: HashMap<(usize, usize), char> = HashMap::new();
    let mut carts: Vec<Cart> = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                ' ' => continue,
                '>' | '<' => {
                    carts.push(Cart {
                        position: (x, y),
                        char: c,
                        turns_made: 0,
                    });
                    track.insert((x, y), '-')
                }
                '^' | 'v' => {
                    carts.push(Cart {
                        position: (x, y),
                        char: c,
                        turns_made: 0,
                    });
                    track.insert((x, y), '|')
                }
                char => track.insert((x, y), char),
            };
        }
    }
    // println!("{:?}", track);
    // println!("{:?}", carts);
    loop {
        for cart in &carts {}
    }
}
