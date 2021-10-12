mod input;

fn main() {
    let text = input::get_input();
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
    let split = split_text(text, "\n\n");

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

fn split_text<'a>(text: &'a str, split_text: &str) -> Vec<&'a str> {
    let split = text.split(split_text)
        .collect::<Vec<&str>>();
    
    return split;
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
    let split = split_text(passport, " ");

    for field in split {
        if !check_field(field.trim()) {
            return false;
        }
    }

    return true;
}

fn check_field(field: &str) -> bool {

    let field_split = split_text(field, ":");

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
        _ => false
    };

    return valid;
}

fn validate_pid(field_val: &str) -> bool {
    todo!()
}

fn validate_ecl(field_val: &str) -> bool {
    todo!()
}

fn validate_hcl(field_val: &str) -> bool {
    todo!()
}

fn validate_hgt(field_val: &str) -> bool {
    todo!()
}

fn validate_eyr(field_val: &str) -> bool {
    todo!()
}

fn validate_iyr(field_val: &str) -> bool {
    todo!()
}

fn validate_byr(field_val: &str) -> bool {
    if field_val.len() != 4 {
        return false;
    };

    let field_val_as_u32: u32 = match field_val.parse() {
        Ok(x) => x,
        Err(e) => return false
    };


    return field_val_as_u32 >= 1920 && field_val_as_u32 <= 2002;
}
