mod input;
use std::collections::HashMap;

struct Policy {
    lower_bound: u32,
    upper_bound: u32,
    letter: u8
}

fn main() {
    let data = &input::get_input();
    
    let validate_passwords_method_one = |data: &str| validate_password_1(data);
    let valid_passwords_1 = validate_passwords(validate_passwords_method_one, data);

    let validate_passwords_method_two = |data: &str| validate_password_2(data);
    let valid_passwords_2 = validate_passwords(validate_passwords_method_two, data);

    println!("Number of valid passwords with method1: {}", valid_passwords_1);
    println!("Number of valid passwords with method2: {}", valid_passwords_2);

}

fn validate_passwords<F>(f:F, data: &Vec<&str>) -> u32 where
    F: Fn(&str) -> bool {

    let mut valid_password_count = 0u32;

    for input in data {
        if f(input) {
            valid_password_count += 1;
        }
    }

    return valid_password_count;
}

fn validate_password_1(input: &str) -> bool {

    let (policy_str, password) = extract_policy_and_password(input);

    let policy = parse_policy(policy_str);

    let mut map = HashMap::<&u8,u32>::new();

    for k in password.as_bytes() {
        let count = map.entry(k).or_insert(0);
        *count += 1;
    }

    if map.contains_key(&policy.letter) {
        let count = match map.get(&policy.letter) {
            Some(v) => v,
            None => panic!("This shouldn't be possible!")
        };

        if *count >= policy.lower_bound && 
           *count <= policy.upper_bound {
            return true;
        }
    }

    return false;
}

fn validate_password_2(input: &str) -> bool {

    let (policy_str, password) = extract_policy_and_password(input);

    let policy = parse_policy(policy_str);

    let password_bytes = password.as_bytes();

    let first_pos = password_bytes[(policy.lower_bound - 1) as usize];
    let second_pos = password_bytes[(policy.upper_bound - 1) as usize];

    println!("{} {} {}", policy.letter, first_pos, second_pos);

    let result = (first_pos == policy.letter &&
        second_pos != policy.letter) ||
        (second_pos == policy.letter &&
        first_pos != policy.letter);

    return result;
}

fn extract_policy_and_password(input: &str) -> (&str, &str) {

    let split = input.split(":");

    let vec = split.collect::<Vec<&str>>();

    return (vec[0], &vec[1][1..]);
}

fn parse_policy(policy_str: &str) -> Policy {

    let split = policy_str.split(" ")
                          .collect::<Vec<&str>>();

    let range = split[0];
    let letter = split[1].as_bytes()[0];

    let range = range.split("-")
                              .collect::<Vec<&str>>();

    let lower = range[0].trim()
                        .parse()
                        .expect("lower bound wasn't an integer");

    let upper = range[1].trim()
                        .parse()
                        .expect("upper bound wasn't an integer");

    return Policy {
        lower_bound: lower,
        upper_bound: upper,
        letter: letter
    };
}
