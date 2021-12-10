use std::collections::HashMap;
use std::env;
use std::fs;

fn check_syntax(line: &str) -> (&str, usize) {
    let mut open_par: Vec<char> = vec![];
    let pairs = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let points = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    for c in line.chars() {
        if c == '(' || c == '[' || c == '{' || c == '<' {
            open_par.push(c);
        } else {
            if c != *pairs
                .get(open_par.get(open_par.len() - 1).unwrap())
                .unwrap()
            {
                return ("", *points.get(&c).unwrap());
            } else {
                open_par.remove(open_par.len() - 1);
            }
        }
    }
    return (line, 0);
}

fn auto_complete(line: &str) -> usize {
    let mut open_par: Vec<char> = vec![];
    let mut close_par: Vec<char> = vec![];

    let pairs = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    for c in line.chars() {
        if c == '(' || c == '[' || c == '{' || c == '<' {
            open_par.push(c);
        } else {
            if c == *pairs
                .get(open_par.get(open_par.len() - 1).unwrap())
                .unwrap()
            {
                open_par.remove(open_par.len() - 1);
            }
        }
    }

    for par in open_par {
        close_par.push(*pairs.get(&par).unwrap());
    }
    close_par.reverse();
    return get_point(&close_par);
}

fn get_point(close_par: &Vec<char>) -> usize {
    let mut score = 0;
    let points = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    for par in close_par {
        score = (score * 5) + *points.get(par).unwrap();
    }
    return score;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong");
    let navigation_subsystem: Vec<&str> = contents.split("\n").collect();

    let part_1: usize = navigation_subsystem
        .iter()
        .map(|line| check_syntax(line).1)
        .sum();
    println!("Part 1: {}", part_1);

    let filtered_navigation_subsystem: Vec<&str> = navigation_subsystem
        .iter()
        .map(|line| check_syntax(line).0)
        .filter(|&s| s != "")
        .collect();
    let mut part_2: Vec<usize> = filtered_navigation_subsystem
        .iter()
        .map(|line| auto_complete(line))
        .collect();
    part_2.sort();
    println!("Part 2: {:?}", part_2[part_2.len() / 2]);
}
