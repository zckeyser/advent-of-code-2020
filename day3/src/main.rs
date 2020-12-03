use std::fs;

fn generate_map(input_map: Vec<&str>, expected_path_length: i32) -> Vec<Vec<bool>> {
    let mut map: Vec<Vec<bool>> = Vec::new();

    for line in input_map {
        let row_in_bools: Vec<bool> = line.chars().map(|c| c == '#').collect();
        let extended_row: Vec<bool> = row_in_bools.into_iter().cycle().take(expected_path_length as usize).collect();
        map.push(extended_row);
    }

    return map;
}

fn main() {
    let filename = "test_input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let slopes_to_test = [
        (1usize, 1usize),
        (3usize, 1usize),
        (5usize, 1usize),
        (7usize, 1usize),
        (1usize, 2usize)
    ];
    let mut hit_counts: Vec<i32> = Vec::new();

    for (x_delta_ptr, y_delta_ptr) in slopes_to_test.iter() {
        let x_delta = *x_delta_ptr;
        let y_delta = *y_delta_ptr;

        let lines: Vec<&str> = contents.split("\n").filter(|x| !x.is_empty()).collect();
        let height = lines.len();

        let req_moves: i32 = ((height as f64) / (y_delta as f64)) as i32;
        let req_length: i32 = req_moves * (x_delta as i32);

        // generate a full map of the paths with the length we'll need horizontally + true/false instead of char mapping
        let map = generate_map(lines, req_length);

        let mut x = 0usize;
        let mut y = 0usize;
        let mut hits = 0;

        for _ in 1..req_moves {
            // move to new position
            // we never hit 0, 0 in the example so the flow is to move then check for a hit
            x += x_delta;
            y += y_delta;

            if map[y][x] {
                hits += 1;
            }
        }
        
        hit_counts.push(hits);
        println!("Trees hit ({}, {}): {}", x_delta, y_delta, hits);
    }

    let mut part2Answer = 1i64;
    for hit_count in hit_counts.into_iter() {
        part2Answer *= hit_count as i64;
    }
    println!("Total multiplied hits (Part 2): {}", part2Answer);
}
