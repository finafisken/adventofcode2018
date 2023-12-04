use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq, Eq)]
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

    fn turn(&mut self, corner: char) {
        match corner {
            '/' => match self.char {
                '<' => self.char = 'v',
                '^' => self.char = '>',
                '>' => self.char = '^',
                'v' => self.char = '<',
                _ => panic!("unknown cart char"),
            },
            '\\' => match self.char {
                '>' => self.char = 'v',
                '^' => self.char = '<',
                '<' => self.char = '^',
                'v' => self.char = '>',
                _ => panic!("unknown cart char"),
            },
            _ => unreachable!(),
        }
    }

    fn next_pos(&self) -> (usize, usize) {
        let (x, y) = self.position;
        match self.char {
            '<' => (x - 1, y),
            '^' => (x, y - 1),
            '>' => (x + 1, y),
            'v' => (x, y + 1),
            _ => panic!("unknown cart char"),
        }
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let distance_self = self.position.0 + self.position.1;
        let distance_other = other.position.0 + other.position.1;

        distance_self.cmp(&distance_other)
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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
    let mut crash = false;

    while !crash {
        for cart in &mut carts {
            let (next_x, next_y) = cart.next_pos();
            let next_char = *track.get(&(next_x, next_y)).unwrap();
            match next_char {
                '/' | '\\' => cart.turn(next_char),
                // can have following cart in same dir?
                '<' | '>' | '^' | 'v' => {
                    crash = true;
                    println!("{:?}", cart.position);
                }
                '+' => cart.intersect(),
                _ => (), // '|' | '-' =>
            }
            cart.position = (next_x, next_y);
        }

        // sort carts for next tick
        carts.sort();
        println!("{:?}", carts);
    }
}
