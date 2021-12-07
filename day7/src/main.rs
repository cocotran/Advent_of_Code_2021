use std::env;
use std::fs;

fn partial_sums(n: i64) -> i64 {
    return (n * (n + 1)) / 2;
}

fn dist_sum(point: i64, points: &Vec<i64>, part: usize) -> i64 {
    let mut sum = 0;
    for i in points {
        if part == 1 {
            sum += (point - i).abs();
        } else if part == 2 {
            sum += partial_sums((point - i).abs());
        }
    }
    return sum;
}

fn geometric_median(points: &Vec<i64>, min: i64, max: i64, part: usize) -> (i64, i64) {
    let mut current_point = 0;

    for point in points {
        current_point += point;
    }

    // Here current_point becomes the
    // Geographic MidPoint
    // Or Center of Gravity of equal
    // discrete mass distributions
    current_point /= points.len() as i64;

    // minimum_distance becomes sum of
    // all distances from MidPoint to
    // all given points
    let mut minimum_distance = dist_sum(current_point, points, part);

    for k in min..(max + 1) {
        let newd = dist_sum(k, points, part);
        if newd < minimum_distance {
            minimum_distance = newd;
            current_point = k;
        }
    }

    return (current_point, minimum_distance);
}

fn get_min_max(points: &Vec<i64>) -> (i64, i64) {
    let mut min = i64::MAX;
    let mut max = 0;
    for &i in points {
        if i < min {
            min = i;
        }
        if i > max {
            max = i;
        }
    }
    return (min, max);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong");
    let mut crabs: Vec<&str> = contents.split(",").collect();
    let crabs: Vec<i64> = crabs.iter_mut().map(|crab| crab.parse().unwrap()).collect();
    let (min, max) = get_min_max(&crabs);

    println!("Part 1: {:?}", geometric_median(&crabs, min, max, 1));
    println!("Part 2: {:?}", geometric_median(&crabs, min, max, 2));
}
