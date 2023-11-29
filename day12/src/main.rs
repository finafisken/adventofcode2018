use std::fs::read_to_string;
fn main() {
    let input = read_to_string("input.txt").unwrap();
    let initial_state = input.lines().next().unwrap().replace("initial state: ", "");
    println!("{}", initial_state);

    for line in input.lines().skip(2) {
        println!("{}", line);
    }
}
