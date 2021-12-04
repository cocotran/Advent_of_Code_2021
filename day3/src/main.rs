use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong");
    let report: Vec<&str> = contents.split("\n").collect();
    let digit = report[0].len();

    let mut rate: Vec<i64> = vec![];
    for _ in 0..digit {
        rate.push(0);
    }

    for &line in &report {
        let mut count = 0;
        for i in line.chars() {
            if i == '0' {
                rate[count] += 1;
            }
            count += 1;
        }
    }

    let mut gamma = String::from("");
    let mut epsilon = String::from("");
    let half = report.len() / 2;
    for &i in &rate {
        if i > half as i64 {
            gamma += "0";
            epsilon += "1";
        } else {
            gamma += "1";
            epsilon += "0";
        }
    }
    let first_bit_oxygen = gamma.chars().nth(0).unwrap();
    let first_bit_co2 = epsilon.chars().nth(0).unwrap();
    let gamma = isize::from_str_radix(&gamma[..], 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon[..], 2).unwrap();

    println!("Part 1: {}", gamma * epsilon);

    let oxygen: isize = get_rating(&report, 0, first_bit_oxygen, "oxygen");
    let co2: isize = get_rating(&report, 0, first_bit_co2, "co2");

    println!("Part 2: {}", oxygen * co2);
}

fn get_rating(vector: &Vec<&str>, bit: i32, bit_value: char, rating_type: &str) -> isize {
    if vector.len() == 1 {
        return isize::from_str_radix(vector[0], 2).unwrap();
    }

    // Get all numbers with the correct bit
    let mut new_vector: Vec<&str> = vec![];
    for &line in vector {
        if line.chars().nth(bit as usize).unwrap() == bit_value {
            new_vector.push(line);
        }
    }

    // Get the most common value
    let mut count_0 = 0;
    let bit_location: usize = (bit + 1).try_into().unwrap();
    if vector.len() > 2 {
        // Prevent index out of bound
        for &line in &new_vector {
            if line.chars().nth(bit_location).unwrap() == '0' {
                count_0 += 1;
            }
        }
    }

    let new_bit_value_oxygen: char = if count_0 > (new_vector.len() / 2) {
        '0'
    } else {
        '1'
    };
    let new_bit_value_co2: char = if count_0 > (new_vector.len() / 2) {
        '1'
    } else {
        '0'
    };
    let new_bit_value: char = if rating_type == "oxygen" {
        new_bit_value_oxygen
    } else {
        new_bit_value_co2
    };

    return get_rating(&new_vector, bit + 1, new_bit_value, rating_type);
}
