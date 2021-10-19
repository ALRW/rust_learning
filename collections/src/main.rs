mod basic_maths;
mod pig_latin;

fn main() {
    let l = vec![1,4,6,2,3,6,6,3,5,2,8,2,10];
    println!(
        "average: {}, median: {}, mode: {}",
        basic_maths::avg(&l),
        basic_maths::median(&l),
        basic_maths::mode(&l)
        );
    let s = String::from("hello my fair lady");
    println!(
        "{}",
        pig_latin::to_pig_latin(s)
        )
}
