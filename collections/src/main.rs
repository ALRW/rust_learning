use std::collections::HashMap;

fn avg_med_mode(mut l: Vec<i32>) -> (i32, i32, i32) {
    l.sort();
    let avg: i32 = l.iter().sum::<i32>() / l.len() as i32;
    let median: i32 = l[(l.len()/2)];
    let mut numbers = HashMap::new();
    for n in l {
        let c = numbers.entry(n).or_insert(0);
        *c += 1
    }
    let (mut mode, mut times) = (0, 0);
    for (k, v) in numbers {
       if v > times {
           mode = k;
           times = v;
       }
    }
    (avg, median, mode)
}

fn main() {
    let l = vec![1,4,6,2,3,6,6,3,5,2,8,2,10];
    let res = avg_med_mode(l);
    println!("average: {}, median: {}, mode: {}", res.0, res.1, res.2)
}
