mod input;
use std::collections::HashMap;

fn main() {
    let data = input::get_input();
    let target = 2020u32;
    
    let two_sum_product_ans = two_sum_product(data, target);
    println!("two sum product ans = {}", two_sum_product_ans);

    let three_sum_product_ans = three_sum_product(data, target);
    println!("three sum product ans = {}", three_sum_product_ans);
}

fn two_sum_product(data: &[u32], target: u32) -> u32 {
    let (a, b) = two_sum(data, target);

    return a * b;
}

fn two_sum(data: &[u32], target: u32) -> (u32, u32) {
    let mut map = HashMap::new();

    for k in data {
        
        let complement = target - k;
        
        if map.contains_key(&complement) {
            return (*k, complement);
        }

        map.insert(k, k);
    }

    return (0,0);
}

fn three_sum_product(data: &[u32], target: u32) -> u32 {
    let (a, b, c) = three_sum(data, target);

    return a * b * c;
}

fn three_sum(data: &[u32], target: u32) -> (u32, u32, u32) {
    for (i, k) in data.iter().enumerate() {
        let current_target = target - k;
        let mut map = HashMap::new();

        for n in (i + 1)..data.len() {
            let val = data[n];

            // Don't bother looking up the complement as it won't exist 
            // (and will cause an overflow panic...).
            if current_target < val {
                continue;
            }

            let complement = current_target - val;

            if map.contains_key(&complement) {
                return (*k, val, complement);
            }

            map.insert(val, val);
        } 
    }

    return (0, 0, 0);
}