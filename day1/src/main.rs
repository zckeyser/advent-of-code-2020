use std::fs;
use itertools::Itertools;

fn main() {
    let filename = "test_input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines = contents.split("\n");
    let filtered_lines = lines.filter(|x| !x.is_empty());
    let ints = filtered_lines.map(|x| x.parse::<i32>().unwrap());
    let combinations: Vec<Vec<i32>> = ints.into_iter().combinations(3).collect();

    for combo in combinations {
        if combo[0] + combo[1] + combo[2] == 2020 {
            println!("{}", combo[0] * combo[1] * combo[2]);
        }
    }
}
