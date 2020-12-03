use std::fs;
use regex::Regex;

fn is_password_valid_by_count(lower_bound: i32, upper_bound: i32, expected_char: char, password: &str) -> bool {
    let mut char_count: i32 = 0;
    for c in password.chars() {
        if c == expected_char {
            char_count += 1;
        }
    }

    return lower_bound <= char_count && upper_bound >= char_count;
}

fn is_password_valid_by_location(a: i32, b: i32, expected_char: char, password: &str) -> bool {
    // the test input is 1-base indexed, whereas in Rust strings are 0-base indexed
    let char_at_a: Option<char> = password.chars().nth((a - 1) as usize);
    let char_at_b: Option<char> = password.chars().nth((b - 1) as usize);

    let has_char_a = char_at_a.is_some() && char_at_a.unwrap() == expected_char;
    let has_char_b = char_at_b.is_some() && char_at_b.unwrap() == expected_char;

    return has_char_a ^ has_char_b;
}

fn main() {    
    let filename = "test_input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").filter(|x| !x.is_empty()).collect();

    let mut valid_count_part1: i32 = 0;
    let mut valid_count_part2: i32 = 0;

    let re = Regex::new(r"^(\d+)-(\d+) (\w): (.+)\s*$").unwrap();

    for line in lines {
        for cap in re.captures_iter(line) {
            let a: i32 = *(&cap[1].parse::<i32>().unwrap());
            let b: i32 = *(&cap[2].parse::<i32>().unwrap());
            let expected_char: char = *(&cap[3][0..1].chars().next().unwrap());
            let string_under_test: &str = &cap[4];

            if is_password_valid_by_count(a, b, expected_char, string_under_test) {
                valid_count_part1 += 1;
            }

            if is_password_valid_by_location(a, b, expected_char, string_under_test) {
                valid_count_part2 += 1;
            }
        }
    }

    println!("Number of valid passwords (Part 1): {}", valid_count_part1);
    println!("Number of valid passwords (Part 2): {}", valid_count_part2);
}
