use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong");
    let measurement: Vec<&str> = contents.split("\n").collect();

    let mut depth = 0;
    let mut horizontal = 0;

    for line in &measurement {
        let command: Vec<&str> = line.clone().split(" ").collect();
        let value = command[1].parse::<i32>().unwrap();
        
        if command[0] == "up" {
            depth -= value;
        }
        else if command[0] == "down" {
            depth += value;
        }
        else if command[0] == "forward" {
            horizontal += value;
        }
    }

    println!("Part 1: {}", depth * horizontal);

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim =  0;

    for line in &measurement {
        let command: Vec<&str> = line.clone().split(" ").collect();
        let value = command[1].parse::<i32>().unwrap();

        if command[0] == "up" {
            aim -= value
        }
        else if command[0] == "down" {
            aim += value
        }
        else if command[0] == "forward" {
            let x = value;
            horizontal += x;
            depth += aim * x;
        }
    }

    println!("Part 2: {}", depth * horizontal);
}
