use std::env;
use std::fs;

fn sliding_window(map: &Vec<Vec<u32>>) -> (Vec<u32>, Vec<(usize, usize)>) {
    let mut result: Vec<Vec<u32>> = vec![];
    // Horizontal
    for line in map {
        let mut arr: Vec<u32> = vec![];
        for (i, &num) in line.iter().enumerate() {
            if i == 0 {
                if num < line[i + 1] {
                    arr.push(num);
                } else {
                    arr.push(9);
                }
            } else if i == (line.len() - 1) {
                if num < line[i - 1] {
                    arr.push(num);
                } else {
                    arr.push(9);
                }
            } else {
                if num < line[i - 1] && num < line[i + 1] {
                    arr.push(num);
                } else {
                    arr.push(9);
                }
            }
        }
        result.push(arr);
    }

    let mut coordinate: Vec<(usize, usize)> = vec![];
    let mut count: Vec<u32> = vec![];
    // Verical
    for (i, line) in result.iter().enumerate() {
        for (j, &num) in line.iter().enumerate() {
            if num != 9 {
                if i == 0 {
                    if num < map[i + 1][j] {
                        count.push(num);
                        coordinate.push((i, j));
                    }
                } else if i == (map.len() - 1) {
                    if num < map[i - 1][j] {
                        count.push(num);
                        coordinate.push((i, j));
                    }
                } else {
                    if num < map[i + 1][j] && num < map[i - 1][j] {
                        count.push(num);
                        coordinate.push((i, j));
                    }
                }
            }
        }
    }
    return (count, coordinate);
}

fn explore(map: &Vec<Vec<u32>>, x: usize, y: usize, visited: &mut Vec<(usize, usize)>) -> usize {
    // Recursive Flood-fill algorithm 
    if visited.contains(&(x, y)) || map[x][y] == 9 {
        return 0;
    } else {
        visited.push((x, y));
    }

    explore(map, x + 1, y, visited);
    explore(map, x - 1, y, visited);
    explore(map, x, y + 1, visited);
    explore(map, x, y - 1, visited);

    return visited.len();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong");
    let mut heightmap: Vec<&str> = contents.split("\n").collect();
    let mut heightmap: Vec<Vec<u32>> = heightmap
        .iter_mut()
        .map(|line| {
            let mut result: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
            result.insert(0, 9);
            result.push(9);
            return result;
        })
        .collect();

    // Pad top
    heightmap.insert(0, vec![9; heightmap[0].len()]);
    // Pad bottom
    heightmap.push(vec![9; heightmap[0].len()]);

    let result = sliding_window(&heightmap);
    let mut count = 0;
    result.0.iter().for_each(|i| count += i + 1);
    println!("Part 1: {}", count);

    let mut part_2: Vec<usize> = result
        .1
        .iter()
        .map(|c| explore(&heightmap, c.0, c.1, &mut vec![]))
        .collect();
    part_2.sort();
    part_2.reverse();

    println!("Part 2: {:?}", part_2[0] * part_2[1] * part_2[2]);
}
