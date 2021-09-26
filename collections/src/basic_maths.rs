use std::collections::HashMap;

pub fn avg(l: &Vec<i32>) -> i32 {
    l.iter().sum::<i32>() / l.len() as i32
}

pub fn median(l: &Vec<i32>) -> i32 {
    let mut sorted = l.to_vec();
    sorted.sort();
    sorted[sorted.len()/2]
}

pub fn mode(l: &Vec<i32>) -> i32 {
    let mut numbers = HashMap::new();
    for n in l {
        let c = numbers.entry(n).or_insert(0);
        *c += 1
    }
    let (mut mode, mut times) = (0, 0);
    for (k, v) in numbers {
        if v > times {
            mode = *k;
            times = v;
        }
    }
    mode
}
