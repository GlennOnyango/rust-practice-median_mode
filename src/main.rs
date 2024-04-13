use std::collections::{btree_map::Values, HashMap};

fn main() {
    let mut number_list = vec![21, 34, 654, 12, 43, 786, 45, 33, 56, 32, 24, 43];

    number_list.sort();

    let middle_index = &number_list.len() / 2;

    let median = &number_list[middle_index];

    let mut occurence = HashMap::new();

    for i in &number_list {
        let count = occurence.entry(i).or_insert(0);
        *count += 1;
    }

    println!("The sorted vector is {:?}", number_list);
    println!("Median is {median}");

    let mut max = 0;
    let mut key_max: &i32 = &0;

    for (key, values) in occurence {
        if values > max {
            max = values;
            key_max = key;
        }
    }

    println!("Want to see count {}", key_max);
}
