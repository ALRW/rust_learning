mod basic_maths;

fn main() {
    let l = vec![1,4,6,2,3,6,6,3,5,2,8,2,10];
    println!(
        "average: {}, median: {}, mode: {}",
        basic_maths::avg(&l),
        basic_maths::median(&l),
        basic_maths::mode(&l)
        )
}
