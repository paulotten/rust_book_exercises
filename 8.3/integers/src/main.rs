use std::collections::HashMap;

fn sum(ints: &Vec<i32>) -> i32 {
    let mut sum = 0;

    for i in ints.iter() {
        sum += i;
    }

    sum
}

fn mean(ints: &Vec<i32>) -> f64 {
    let total = sum(ints) as f64;
    let count = ints.len() as f64;
    
    total / count
}

fn mode(ints: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    
    for i in ints.iter() {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    
    let mut mode = 0;
    let mut count = 0;
    
    for (i, c) in &map {
        if c > &count {
            mode = **i;
            count = *c;
        }
    }
    
    mode
}

fn median(ints: &Vec<i32>) -> i32 {
    let mut sort_me = ints.to_vec();
    sort_me.sort();
    
    sort_me[ints.len() / 2]
}

fn main() {
    let ints = vec![1, 80, 1, 41, 4];
    println!("{:?}", ints);

    println!("{}", sum(&ints));
    println!("{}", mean(&ints));
    println!("{}", mode(&ints));
    println!("{}", median(&ints));
}
