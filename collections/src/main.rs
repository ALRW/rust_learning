use std::collections::HashMap;
use std::io;

mod basic_maths;
mod pig_latin;

fn add_department_member(departments: &mut HashMap<&String, Vec<&String>>) {
    let mut answer = String::new();
    println!("What is their department and name?");
    match io::stdin().read_line(&mut answer) {
        Ok(_) => {
            let v: Vec<&str> = answer.split(' ').collect();
            for n in v {
                println!("result so far {}", n);
            }

        }
        _ => { println!("There was an issue with your input, try again.") }
    }
}

fn department_loop() {
    let mut departments: HashMap<&String, Vec<&String>> = HashMap::new();
    loop {
        let mut input = String::new();
        println!("Welcome, please select from the following options:
        1. To view a department.
        2. To add a new team member.
        3. To Exit."
        );
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.as_str().trim() {
                    "1" => { print!("One") }
                    "2" => {
                        add_department_member(&mut departments)
                    }
                    _ => {
                        println!("Input {}", input);
                        break
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break
            }
        }
    }
}

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
    );

    department_loop();
}
