use std::{collections::HashMap, fs::read_to_string};

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

    let mut state = initial_state.clone();
    let mut offset = 0;
    for _gen in 1..=20 {
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
        state = new_state;
    }
    let sum_pots: isize = state.chars().enumerate().fold(0, |sum, (i, val)| {
        if val == '#' {
            return sum + i as isize + offset;
        }
        sum
    });

    println!("{}", sum_pots);
}
