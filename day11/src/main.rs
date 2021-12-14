use std::env;
use std::fs;

struct Octopus {
    energy: u32,
    flash: bool,
}

impl Octopus {
    fn level_up(&mut self) -> bool {
        if !self.flash {
            if self.energy == 9 {
                self.energy = 0;
                self.flash = true;
                return true;
            } else {
                self.energy += 1;
            }
        }
        return false;
    }

    fn reset_flash(&mut self) {
        self.flash = false;
    }
}

fn step(octopi: &mut Vec<Vec<Octopus>>) -> usize {
    let mut flashes: Vec<(usize, usize)> = vec![];
    for (i, line) in octopi.iter_mut().enumerate() {
        for (j, octopus) in line.iter_mut().enumerate() {
            if octopus.level_up() {
                flashes.push((i, j));
            }
        }
    }

    let mut count = flashes.len();
    let mut len = flashes.len();

    while len > 0 {
        let mut flashes_clone: Vec<(usize, usize)> = vec![];
        for flash in &mut flashes {
            let y = flash.0;
            let x = flash.1;
            if octopi[y + 1][x].level_up() {
                count += 1;
                flashes_clone.push((y + 1, x));
            }
            if octopi[y - 1][x].level_up() {
                count += 1;
                flashes_clone.push((y - 1, x));
            }
            if octopi[y][x - 1].level_up() {
                count += 1;
                flashes_clone.push((y, x - 1));
            }
            if octopi[y][x + 1].level_up() {
                count += 1;
                flashes_clone.push((y, x + 1));
            }
            if octopi[y + 1][x - 1].level_up() {
                count += 1;
                flashes_clone.push((y + 1, x - 1));
            }
            if octopi[y + 1][x + 1].level_up() {
                count += 1;
                flashes_clone.push((y + 1, x + 1));
            }
            if octopi[y - 1][x - 1].level_up() {
                count += 1;
                flashes_clone.push((y - 1, x - 1));
            }
            if octopi[y - 1][x + 1].level_up() {
                count += 1;
                flashes_clone.push((y - 1, x + 1));
            }
        }
        flashes = flashes_clone;
        len = flashes.len();
    }

    return count;
}

fn reset_flash(octopi: &mut Vec<Vec<Octopus>>) {
    for i in 1..11 {
        for j in 1..11 {
            octopi[i][j].reset_flash();
        }
    }
}

fn check_flash_all(octopi: &Vec<Vec<Octopus>>) -> bool {
    for i in octopi {
        for octopus in i {
            if octopus.energy != 0 {
                return false;
            }
        }
    }
    return true;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong");
    let mut octopi: Vec<&str> = contents.split("\n").collect();

    octopi.insert(0, "0000000000");
    octopi.push("0000000000");

    let mut octopi: Vec<Vec<Octopus>> = octopi
        .iter_mut()
        .map(|o| {
            let mut result: Vec<Octopus> = o
                .chars()
                .map(|c| Octopus {
                    energy: c.to_digit(10).unwrap(),
                    flash: if c == '0' { true } else { false },
                })
                .collect();
            result.insert(
                0,
                Octopus {
                    energy: 0,
                    flash: true,
                },
            );
            result.push(Octopus {
                energy: 0,
                flash: true,
            });
            return result;
        })
        .collect();

    let mut count = 0;
    for _ in 0..(args[2].parse().unwrap()) {
        count += step(&mut octopi);
        reset_flash(&mut octopi);
    }
    println!("Part 1: {}", count);

    let mut count = 0;
    loop {
        count += 1;
        step(&mut octopi);
        if check_flash_all(&octopi) {
            break;
        }
        reset_flash(&mut octopi);
    }
    println!("Part 2: {}", count);
}
