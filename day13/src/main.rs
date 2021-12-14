use std::env;
use std::fs;

fn get_reflection_point((p, q): (i64, i64), (a, b, c): (i64, i64, i64)) -> (i64, i64) {
    // https://math.stackexchange.com/a/1743581
    let x: i64 = (p * (a.pow(2) - b.pow(2)) - 2 * b * (a * q + c)) / (a.pow(2) + b.pow(2));
    let y: i64 = (q * (b.pow(2) - a.pow(2)) - 2 * a * (b * p + c)) / (a.pow(2) + b.pow(2));
    return (x, y);
}

fn get_equation(s: &str) -> (i64, i64, i64) {
    let command: Vec<&str> = s.split("=").collect();
    let c: i64 = command[1].parse().unwrap();
    return (
        if command[0] == "x" { 0 } else { 1 },
        if command[0] == "x" { 1 } else { 0 },
        -1 * c,
    );
}

fn fold(coordinates: &mut Vec<(i64, i64)>, line: (i64, i64, i64)) -> Vec<(i64, i64)> {
    let mut new_coordinates: Vec<(i64, i64)> = vec![];
    let direction: &str = if line.0 == 0 {
        "vertical"
    } else {
        "horizontal"
    };
    for &mut point in coordinates {
        if direction == "vertical" {
            if point.0 < line.2.abs() {
                new_coordinates.push(point);
            } else {
                let new_point: (i64, i64) = get_reflection_point(point, line);
                new_coordinates.push(new_point);
            }
        } else if direction == "horizontal" {
            if point.1 < line.2.abs() {
                new_coordinates.push(point);
            } else {
                let new_point: (i64, i64) = get_reflection_point(point, line);
                new_coordinates.push(new_point);
            }
        }
    }
    new_coordinates.sort();
    new_coordinates.dedup();
    return new_coordinates;
}

fn get_dimension(coordinates: &Vec<(i64, i64)>) -> (usize, usize) {
    // Get dimension
    let mut width = 0;
    let mut height = 0;
    for i in coordinates {
        if i.0 > width {
            width = i.0;
        }
        if i.1 > height {
            height = i.1;
        }
    }
    return (width as usize, height as usize);
}

fn draw(coordinates: &Vec<(i64, i64)>, dimension: (usize, usize)) {
    let mut map: Vec<Vec<&str>> = vec![vec![" "; dimension.0 + 1]; dimension.1 + 1];
    for point in coordinates {
        map[point.1 as usize][point.0 as usize] = "#";
    }

    for i in map {
        for j in i {
            print!("{}", j);
        }
        println!("");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong");
    let mut input: Vec<&str> = contents.split("\n\n").collect();

    let mut coordinates: Vec<&str> = input[0].split("\n").collect();
    let mut coordinates: Vec<(i64, i64)> = coordinates
        .iter_mut()
        .map(|i| {
            let coordinate: Vec<&str> = i.split(",").collect();
            return (
                coordinate[0].parse().unwrap(),
                coordinate[1].parse().unwrap(),
            );
        })
        .collect();

    let mut fold_commands: Vec<&str> = input[1].split("\n").collect();
    let fold_commands: Vec<(i64, i64, i64)> = fold_commands
        .iter_mut()
        .map(|i| {
            let command: Vec<&str> = i.split(" ").collect();
            return get_equation(command[2]);
        })
        .collect();

    for command in fold_commands {
        coordinates = fold(&mut coordinates, command);
    }

    // println!("{:?}", coordinates.len());

    draw(&coordinates, get_dimension(&coordinates));
}
