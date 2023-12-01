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
    for _gen in 1..=20 {
        let mut new_state = String::from("..");
        new_state.push_str(state.as_str());
        new_state.push_str("..");
        let extended_state = new_state.clone();

        for i in 2..new_state.len() - 2 {
            let to_match = &extended_state[i - 2..=i + 2];
            if match_results.contains_key(to_match) {
                new_state = new_state
                    .chars()
                    .enumerate()
                    .fold(Vec::new(), |mut acc, (ii, char)| {
                        if ii == i {
                            acc.push(*match_results.get(to_match).unwrap())
                        } else {
                            acc.push(char)
                        }
                        acc
                    })
                    .into_iter()
                    .collect::<String>();
            }
            //  else {
            //     new_state = new_state
            //         .chars()
            //         .enumerate()
            //         .fold(Vec::new(), |mut acc, (ii, char)| {
            //             if ii == i {
            //                 acc.push('.')
            //             } else {
            //                 acc.push(char)
            //             }
            //             acc
            //         })
            //         .into_iter()
            //         .collect::<String>();
            // }
        }

        println!("gen {}: {}", _gen, new_state);
        state = new_state;
    }
    let sum_pots: isize = state.chars().enumerate().fold(0, |sum, (i, val)| {
        if val == '#' {
            return sum + i as isize - 40;
        }
        sum
    });

    println!("{}", sum_pots);
}
