use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong");
    let measurement: Vec<&str> = contents.split("\n").collect();

    let mut count = 0;
    for i in 1..measurement.len() {
        if &measurement[i].parse::<i32>().unwrap() > &measurement[i - 1].parse::<i32>().unwrap() {
            count += 1;
        }
    }

    println!("Part 1: {}", count);

    let mut first_window = &measurement[0].parse::<i32>().unwrap()
        + &measurement[1].parse::<i32>().unwrap()
        + &measurement[2].parse::<i32>().unwrap();
    let mut count = 0;
    for i in 1..(measurement.len() - 2) {
        let second_window = &measurement[i].parse::<i32>().unwrap()
            + &measurement[i + 1].parse::<i32>().unwrap()
            + &measurement[i + 2].parse::<i32>().unwrap();
        if second_window > first_window {
            count += 1;
        }
        first_window = second_window;
    }

    println!("Part 2: {}", count);
}
