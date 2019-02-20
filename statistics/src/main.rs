use std::collections::HashMap;

fn statistics(v: Vec<i32>) -> (i32, i32, i32) {
    let mut v: Vec<i32> = v.clone();
    v.sort();
    let sum: i32 = v.iter().sum();
    let mean = if v.len() > 0 { sum / v.len() as i32 } else { 0 };
    let median = if v.len() > 0 { v[v.len() / 2] } else { 0 };
    let mut hash_map = HashMap::new();
    for i in v {
        let count = hash_map.entry(i).or_insert(0);
        *count += 1;
    }
    let max_count = hash_map.values().max().unwrap_or(&1);
    let max_key = hash_map
        .iter()
        .find(|i| *i.1 == *max_count)
        .unwrap_or((&0, &0))
        .0;
    let mode = *max_key;
    (mean, median, mode)
}

fn main() {
    println!("{:?}", statistics(vec![]));
    println!("{:?}", statistics(vec![1, 2, 3, 4]));
    println!("{:?}", statistics(vec![3, 1, 4, 1, 5, 9, 2, 6]));
}
