use std::env;
use std::fs;

fn count_unique_number(entries: &Vec<&str>) -> usize {
    let mut count = 0;
    for &line in entries {
        let entry: Vec<&str> = line.split("|").collect();
        let output_values: Vec<&str> = entry[1].split(" ").collect();
        for value in output_values {
            if value.len() == 2 || value.len() == 4 || value.len() == 3 || value.len() == 7 {
                count += 1;
            }
        }
    }
    return count;
}

fn parse_number(patterns: &Vec<&str>, values: &Vec<&str>) -> usize {
    let mut one = "";
    let mut four = "";

    // Figure out 1 and 4
    for &signal in patterns {
        if signal.len() == 2 {
            one = signal;
        } else if signal.len() == 4 {
            four = signal;
        }
    }

    // Get top right segment wire
    let mut l: String = String::from("");
    for i in four.chars() {
        if !one.contains(i) {
            l += &i.to_string();
        }
    }

    let l_segment_1: char = l.as_bytes()[0] as char;
    let l_segment_2: char = l.as_bytes()[1] as char;
    let one_1: char = one.as_bytes()[0] as char;
    let one_2: char = one.as_bytes()[1] as char;

    // Process output
    let mut number: String = String::from("");
    for &i in values {
        if i.len() == 2 {
            number += "1";
        } else if i.len() == 4 {
            number += "4";
        } else if i.len() == 3 {
            number += "7";
        } else if i.len() == 7 {
            number += "8";
        } else if i.len() == 5 {
            if i.contains(one_1) && i.contains(one_2) {
                number += "3";
            } else if i.contains(l_segment_1) && i.contains(l_segment_2) {
                number += "5";
            } else {
                number += "2";
            }
        } else if i.len() == 6 {
            if i.contains(one_1) && i.contains(one_2) {
                if i.contains(l_segment_1) && i.contains(l_segment_2) {
                    number += "9";
                } else {
                    number += "0";
                }
            } else {
                number += "6";
            }
        }
    }

    return number.parse().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong");
    let entries: Vec<&str> = contents.split("\n").collect();

    println!("Part 1: {}", count_unique_number(&entries));

    let mut count = 0;
    for line in &entries {
        let entry: Vec<&str> = line.split("|").collect();
        let patterns: Vec<&str> = entry[0].trim_end().split(" ").collect();
        let output_values: Vec<&str> = entry[1].trim_start().split(" ").collect();
        count += parse_number(&patterns, &output_values);
    }

    println!("Part 2: {}", count);
}
