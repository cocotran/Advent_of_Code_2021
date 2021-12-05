use std::env;
use std::fs;

struct Diagram {
    map: Vec<Vec<i64>>,
}

impl Diagram {
    fn plot(&mut self, x: i64, y: i64) {
        self.map[y as usize][x as usize] += 1;
    }

    fn plot_line_low(&mut self, x0: i64, y0: i64, x1: i64, y1: i64) {
        let dx: i64 = x1 - x0;
        let mut dy: i64 = y1 - y0;
        let mut yi: i64 = 1;

        if dy < 0 {
            yi = -1;
            dy = -dy;
        }

        let mut d = 2 * dy - dx;
        let mut y = y0;

        for x in x0..(x1 + 1) {
            self.plot(x, y);
            if d > 0 {
                y += yi;
                d += 2 * (dy - dx);
            } else {
                d += 2 * dy;
            }
        }
    }

    fn plot_line_high(&mut self, x0: i64, y0: i64, x1: i64, y1: i64) {
        let mut dx: i64 = x1 - x0;
        let dy: i64 = y1 - y0;
        let mut xi: i64 = 1;

        if dx < 0 {
            xi = -1;
            dx = -dx;
        }

        let mut d = 2 * dx - dy;
        let mut x = x0;

        for y in y0..(y1 + 1) {
            self.plot(x, y);
            if d > 0 {
                x += xi;
                d += 2 * (dx - dy);
            } else {
                d += 2 * dx;
            }
        }
    }

    fn plot_line(&mut self, x0: i64, y0: i64, x1: i64, y1: i64) {
        // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
        if (y1 - y0).abs() < (x1 - x0).abs() {
            if x0 > x1 {
                self.plot_line_low(x1, y1, x0, y0);
            } else {
                self.plot_line_low(x0, y0, x1, y1);
            }
        } else {
            if y0 > y1 {
                self.plot_line_high(x1, y1, x0, y0);
            } else {
                self.plot_line_high(x0, y0, x1, y1);
            }
        }
    }

    fn get_point(&self) -> i64 {
        let mut point = 0;
        self.map.iter().for_each(|line| {
            line.iter().for_each(|&p| {
                if p >= 2 {
                    point += 1
                }
            })
        });
        return point;
    }
}

fn generate_new_diagram((width, height): (i64, i64)) -> Diagram {
    let new_line: Vec<i64> = vec![0; (width + 1) as usize];
    return Diagram {
        map: vec![new_line; (height + 1) as usize],
    };
}

fn get_coordinates(line: &str) -> Vec<i64> {
    let coordinate: Vec<&str> = line.split(" -> ").collect();
    let mut result: Vec<i64> = vec![];
    for i in coordinate {
        let points: Vec<&str> = i.split(",").collect();
        for p in points {
            result.push(p.parse().unwrap());
        }
    }
    return result;
}

fn get_dimension(lines: &Vec<Vec<i64>>) -> (i64, i64) {
    let mut width = 0;
    let mut height = 0;
    for line in lines {
        if line[0] > width {
            width = line[0];
        }
        if line[1] > height {
            height = line[1];
        }
    }
    return (width, height);
}

fn is_vertical_or_horizontal_line(line: &Vec<i64>) -> bool {
    return line[0] == line[2] || line[1] == line[3];
}

fn run(part: usize, lines: &Vec<Vec<i64>>, diagram: &mut Diagram) {
    if part == 1 {
        for line in lines {
            if is_vertical_or_horizontal_line(&line) {
                diagram.plot_line(line[0], line[1], line[2], line[3]);
            }
        }

        let point = diagram.get_point();
        println!("Part 1: {}", point);
    } else if part == 2 {
        for line in lines {
            diagram.plot_line(line[0], line[1], line[2], line[3]);
        }

        let point = diagram.get_point();
        println!("Part 2: {}", point);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong");
    let lines_str: Vec<&str> = contents.split("\n").collect();

    let mut lines: Vec<Vec<i64>> = vec![];

    for line in lines_str {
        lines.push(get_coordinates(line));
    }

    let (width, height) = get_dimension(&lines);
    let mut diagram = generate_new_diagram((width, height));

    // run(1, &lines, &mut diagram);
    // run(2, &lines, &mut diagram);

    // for i in diagram.map {
    //     println!("{:?}", i);
    // }
}
