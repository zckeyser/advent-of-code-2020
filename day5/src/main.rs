use std::fs;

fn main() {    
    let filename = "test_input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").filter(|x| !x.is_empty()).collect();
    
    let mut all_seat_ids: Vec<usize> = Vec::new();
    let mut max_seat_id = 0;

    for line in lines {
        let mut vert_lower_bound = 0.0;
        let mut vert_upper_bound = 127.0;
        let mut hor_lower_bound = 0.0;
        let mut hor_upper_bound = 7.0;

        for c in line.chars() {
            if c == 'B' {
                let midpoint: f64 = (vert_upper_bound + vert_lower_bound) / 2.0;
                let new_bound: f64 = midpoint.ceil();
                vert_lower_bound = new_bound;
            } else if c == 'F'{
                let midpoint: f64 = (vert_upper_bound + vert_lower_bound) / 2.0;
                let new_bound: f64 = midpoint.floor();
                vert_upper_bound = new_bound;
            } else if c == 'R' {
                let midpoint: f64 = (hor_upper_bound + hor_lower_bound) / 2.0;
                let new_bound: f64 = midpoint.ceil();
                hor_lower_bound = new_bound;
            } else if c == 'L' {
                let midpoint: f64 = (hor_upper_bound + hor_lower_bound) / 2.0;
                let new_bound: f64 = midpoint.floor();
                hor_upper_bound = new_bound;
            }
        }

        let seat_id = (vert_upper_bound as usize * 8) + hor_upper_bound as usize;

        all_seat_ids.push(seat_id);

        if seat_id > max_seat_id {
            max_seat_id = seat_id;
        }
    }

    for seat_id in 1..max_seat_id {
        if all_seat_ids.contains(&(seat_id - 1)) && all_seat_ids.contains(&(seat_id + 1)) && !all_seat_ids.contains(&seat_id) {
            println!("Your seat ID is: {}", seat_id);
        }
    }

    println!("The max seat ID is: {}", max_seat_id)
}
