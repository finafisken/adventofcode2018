use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let initial_state = input.lines().next().unwrap().replace("initial state: ", "");

    let mut match_results: HashMap<String, char> = HashMap::new();

    for line in input.lines().skip(2) {
        let [pattern, result]: [&str; 2] = line
            .split(" => ")
            .collect::<Vec<&str>>()
            .try_into()
            .expect("wrong format in input");

        match_results.insert(String::from(pattern), result.chars().next().unwrap());
    }

    // println!("Part 1: {}", part2(20, initial_state, match_results));
    println!(
        "Part 2: {}",
        part2(50000000000, initial_state, match_results)
    );
}

fn part1(gens: u64, initial_state: String, match_results: HashMap<String, char>) -> isize {
    let mut state = initial_state.clone();
    let mut offset = 0;
    for _gen in 1..=gens {
        let mut new_state = String::from("...");
        offset -= 3;
        new_state.push_str(state.as_str());
        new_state.push_str("...");
        let extended_state = new_state.clone();

        for i in 2..new_state.len() - 2 {
            let to_match = &extended_state[i - 2..=i + 2];
            if match_results.contains_key(to_match) {
                new_state = new_state
                    .chars()
                    .enumerate()
                    .map(|(ii, char)| {
                        if ii == i {
                            *match_results.get(to_match).unwrap()
                        } else {
                            char
                        }
                    })
                    .collect::<String>();
            }
        }

        state = new_state;
    }

    sum_pots(&state, offset)
}

fn part2(gens: u64, initial_state: String, match_results: HashMap<String, char>) -> isize {
    let mut state = initial_state.clone();
    let mut offset = 0;
    // let mut latest_results: VecDeque<String> = VecDeque::new();
    let mut seen: HashMap<String, u32> = HashMap::from([(state.clone(), 1_u32)]);
    // let mut result: isize = 0;
    for _gen in 1..=gens {
        let mut new_state = String::from("...");
        offset -= 3;
        new_state.push_str(state.as_str());
        new_state.push_str("...");
        let extended_state = new_state.clone();

        for i in 2..new_state.len() - 2 {
            let to_match = &extended_state[i - 2..=i + 2];
            if match_results.contains_key(to_match) {
                new_state = new_state
                    .chars()
                    .enumerate()
                    .map(|(ii, char)| {
                        if ii == i {
                            *match_results.get(to_match).unwrap()
                        } else {
                            char
                        }
                    })
                    .collect::<String>();
            }
        }

        // println!("gen {}: {}", _gen, new_state);
        //
        // let trimmed = &new_state[2..new_state.len() - 5];
        // println!("{new_state}");
        // println!("{trimmed}");
        let gx = new_state.clone();
        let trimmed = gx.trim_start_matches('.').trim_end_matches('.');

        *seen.entry(trimmed.to_string()).or_insert(0_u32) += 1;
        // result = sum_pots(&state, offset);
        state = new_state;

        if *seen.get(&trimmed.to_string()).unwrap() > 100_u32 {
            break;
        }
        // println!("{}", result);
        // let pots = state.chars().filter(|c| *c == '#').count();

        // if latest_results.len() >= 10000 {
        //     latest_results.pop_front();
        //     if latest_results.iter().all(|r| *r == pots) {
        //         break;
        //     }
        // }
        //
        // latest_results.push_back(pots);
    }
    // result
    seen.keys().map(|state| sum_pots(state, offset))
    // NEED TO KEEP TRACK OF OFFSET SINCE WE TRIM
    sum_pots(&state, offset)
}

fn sum_pots(state: &str, offset: isize) -> isize {
    state.chars().enumerate().fold(0, |sum, (i, val)| {
        if val == '#' {
            return sum + i as isize + offset;
        }
        sum
    })
}
