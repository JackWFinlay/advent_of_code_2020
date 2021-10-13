use std::str;

fn main() {
    
    let text = include_str!("input.txt");

    let compulsory_fields = vec!("byr",
                                          "iyr",
                                          "eyr",
                                          "hgt",
                                          "hcl",
                                          "ecl",
                                          "pid");

    let (present_count, valid_count) = check_passports(&text, &compulsory_fields);

    println!("Passports with all fields present: {}", present_count);
    println!("Passports with all fields valid: {}", valid_count);
}

fn check_passports(text: &str, compulsory_fields: &Vec<&str>) -> (u32, u32) {
    
    let mut present_count: u32 = 0;
    let mut valid_count: u32 = 0;
    let split = text.split("\n\n");//split_text(text, "\n\n");

    for passport in split {
        if check_passport_fields_present(passport, compulsory_fields) {
            present_count += 1;
            if check_passport_valid(passport) {
                valid_count += 1;
            }
        }
    }

    return (present_count, valid_count);
}

fn check_passport_fields_present(passport: &str, compulsory_fields: &Vec<&str>) -> bool {
    
    for field in compulsory_fields {
        if !passport.contains(field) {
            return false;
        }
    }
    
    return true;
}

fn check_passport_valid(passport: &str) -> bool {
    
    let split = passport.split(|c| c == ' ' || c == '\n');

    for field in split {
        if !check_field(field.trim()) {
            return false;
        }
    }

    return true;
}

fn check_field(field: &str) -> bool {

    let field_split = field.split(":")
                        .collect::<Vec<&str>>();

    let field_name = field_split[0].trim();
    let field_val = field_split[1].trim();

    let valid = match field_name {
        "byr" => validate_byr(field_val),
        "iyr" => validate_iyr(field_val),
        "eyr" => validate_eyr(field_val),
        "hgt" => validate_hgt(field_val),
        "hcl" => validate_hcl(field_val),
        "ecl" => validate_ecl(field_val),
        "pid" => validate_pid(field_val),
        "cid" => true,
        _ => false
    };

    return valid;
}

fn validate_pid(field_val: &str) -> bool {
    
    if field_val.len() != 9 {
        return false;
    }

    return match field_val.parse::<u32>() {
        Ok(_) => true,
        Err(_) => false
    };
}

fn validate_ecl(field_val: &str) -> bool {
    
    if field_val.len() != 3 {
        return false;
    }

    let valid_options = &vec!("amb", "blu", "brn", "gry", "grn", "hzl", "oth");

    return is_in_set(field_val, valid_options);
}

fn validate_hcl(field_val: &str) -> bool {
    
    if field_val.len() != 7 {
        return false;
    }

    if !field_val.starts_with("#") {
        return false;
    }

    let valid_options = &vec!("0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
                                      "a", "b", "c", "d", "e","f");

    let hexcode = &field_val[1..];

    // trying to do this without importing the hex crate.
    for digit in hexcode.bytes() {
        let digit_arr = &[digit];
        let digit_str = match str::from_utf8(digit_arr) {
            Ok(x) => x,
            Err(_) => return false
        };

        if !is_in_set(&digit_str, valid_options) {
            return false;
        }
    }

    return true;
}

fn is_in_set(val: &str, valid_options: &Vec<&str>) -> bool {

    let mut valid = false;

    for check_val in valid_options {
        if *check_val == val {
            valid = true;
            break;
        }
    }
    
    return valid;
}

fn validate_hgt(field_val: &str) -> bool {
    
    let height_str = &field_val[..(field_val.len() - 2)];
    let height = match height_str.parse() {
        Ok(x) => x,
        Err(_) => return false
    };
    
    let unit = &field_val[(field_val.len() - 2)..];

    let (lower, upper) = match unit {
        "cm" => (150, 193),
        "in" => (59, 76),
        _ => return false
    };

    return is_in_range(height, lower, upper);
}

fn validate_eyr(field_val: &str) -> bool {
    return validate_years(field_val, 2020, 2030);
}

fn validate_iyr(field_val: &str) -> bool {
    return validate_years(field_val, 2010, 2020);
}

fn validate_byr(field_val: &str) -> bool {
    return validate_years(field_val, 1920, 2002);
}

fn validate_years(val: &str, lower: u32, upper: u32) -> bool {
    
    if val.len() != 4 {
        return false;
    };

    let val_as_u32: u32 = match val.parse() {
        Ok(x) => x,
        Err(_) => return false
    };

    return is_in_range(val_as_u32,lower, upper);
}

fn is_in_range(val: u32, lower: u32, upper: u32) -> bool {
    return val >= lower && val <= upper;
}