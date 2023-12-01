use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let initial_state = input.lines().next().unwrap().replace("initial state: ", "");
    // println!("{}", initial_state);

    let mut match_results: HashMap<String, char> = HashMap::new();

    for line in input.lines().skip(2) {
        let [pattern, _, result]: [&str; 3] = line
            .split(' ')
            .collect::<Vec<&str>>()
            .try_into()
            .expect("wrong format in input");

        match_results.insert(String::from(pattern), result.chars().next().unwrap());
        // println!("{} to {}", pattern, result);
    }

    let mut state = initial_state.clone();
    for _gen in 1..=20 {
        let mut new_state = String::default();
        for chunk in state.chars().collect::<Vec<_>>().windows(5) {
            let part = chunk.iter().collect::<String>();
            if match_results.contains_key(&part) {
                let new_status = match_results.get(&part).expect("OOOF");
                let mut chars: Vec<char> = part.chars().collect();
                chars[2] = *new_status;

                new_state += &chars.iter().collect::<String>();
                // println!("hit {}", new_status);
            } else {
                new_state += &part;
                // println!("miss");
            }
            // println!("{}", chunk.iter().collect::<String>());
        }

        println!("{}", new_state);
        state = new_state;
    }
    let sum_pots = state.chars().enumerate().fold(0, |sum, (i, val)| {
        if val == '#' {
            return sum + i;
        }
        sum
    });

    println!("{}", sum_pots);
}
