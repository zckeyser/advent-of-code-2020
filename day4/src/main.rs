use std::fs;
use regex::Regex;

fn validate_field(field_name: &str, field_value: &str) -> bool {
    if field_name == "byr" || field_name == "iyr" || field_name == "eyr" {
        // fields we validate by converting directly to ints
        let val_as_int_unwrapped = field_value.parse::<i32>();
        
        let val_as_int: i32;

        match val_as_int_unwrapped {
            Ok(v) => val_as_int = v,
            Err(_) => {
                return false;
            }
        }

        if field_name == "byr" {
            return val_as_int >= 1920 && val_as_int <= 2002;
        } else if field_name == "iyr" {
            return val_as_int >= 2010 && val_as_int <= 2020;
        } else if field_name == "eyr" {
            return val_as_int >= 2020 && val_as_int <= 2030;
        } else {
            panic!("I should never be hit! This means that we are not encapsulating the right logic inside the above if statements!");
        }
    } else if field_name == "hgt" {
        // height must be 150 <= x <= 193 for cm or 59 <= x <= 76 for in
        let field_len = field_value.len();
        if field_len < 3 {
            // must be at least 3 chars to be valid given format of <num><unit> where unit is in or cm
            return false;
        }
        
        // pull out the int value
        let height_value_unwrapped = field_value[0..field_len - 2].parse::<i32>();
        let height_value: i32;

        match height_value_unwrapped {
            Ok(h) => height_value = h,
            Err(_) => {
                return false;
            }
        }

        // pull out the unit of measure
        let height_unit: &str = &field_value[field_len - 2..];

        if height_unit == "cm".to_string() {
            return height_value >= 150 && height_value <= 193;
        } else if height_unit == "in" {
            return height_value >= 59 && height_value <= 76;
        } else {
            return false;
        }
    } else if field_name == "hcl" {
        // hcl must be a valid 6-digit hex code (lowercase chars, leading hash sign)
        let pattern = Regex::new(r"^#[abcdef1234567890]{6}$").unwrap();
        
        return pattern.is_match(field_value);
    } else if field_name == "ecl" {
        let valid_eye_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        return valid_eye_colors.contains(&field_value);
    } else if field_name == "pid" {
        // nine digit number, including leading zeroes
        let pattern = Regex::new(r"^[0123456789]{9}$").unwrap();
        
        return pattern.is_match(field_value);
    } else if field_name == "cid" {
        // don't need to validate, but want to correctly return a value for it
        return true;
    } else {
        panic!("You missed a field, dummy!");
    }
}

fn main() {
    // note: cid not required for part 1
    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    
    
    let filename = "test_input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut valid_exists_passports = 0;
    let mut valid_validated_passports = 0;
    let mut passport_fields: std::collections::HashMap<&str, bool> = std::collections::HashMap::new();

    for line in lines.iter() {
        // marks a boundary between passport definitions
        // check if the recently read passport is valid and clear the underlying dict
        if line.is_empty() {
            let mut is_valid_by_fields_exist = true;
            let mut is_valid_by_fields_valid = true;

            for req_field in required_fields.iter() {
                if is_valid_by_fields_exist && !passport_fields.contains_key(req_field) {
                    is_valid_by_fields_exist = false;
                }

                if is_valid_by_fields_valid && (!passport_fields.contains_key(req_field) || !passport_fields.get(req_field).unwrap()) {
                    is_valid_by_fields_valid = false;
                }
            }

            if is_valid_by_fields_exist {
                valid_exists_passports += 1;
            }
            
            if is_valid_by_fields_valid {
                valid_validated_passports += 1;
            }

            // clear the existing passport's info out
            passport_fields.clear();

            continue;
        }

        let fields: Vec<&str> = line.split(' ').collect();
        let fields_with_values: Vec<(&str, &str)> = fields.iter().map(|kv_pair| (&kv_pair[0..3], &kv_pair[4..])).collect();


        for kv in fields_with_values.iter() {
            let (field, value) = kv;

            passport_fields.insert(field, validate_field(field, value));
        }
    }

    println!("Part A: {}", valid_exists_passports);
    println!("Part B: {}", valid_validated_passports);
}
