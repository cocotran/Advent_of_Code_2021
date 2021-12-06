use std::env;
use std::fs;

struct Tracker {
    tracker: Vec<i64>,
}

impl Tracker {
    const WIDTH: usize = 9;

    fn left_rotate_by_one(&mut self) {
        let fishes_with_timer_0 = self.tracker[0];
        for i in 0..(Self::WIDTH - 1) {
            self.tracker[i] = self.tracker[i + 1];
        }

        // Spawn new fishes
        self.tracker[Self::WIDTH - 1] = fishes_with_timer_0;

        // Reset fishes that just give birth
        self.tracker[6] += fishes_with_timer_0;
    }

    fn get_number_of_fish(&self) -> i64 {
        let mut fishes = 0;
        for i in &self.tracker {
            fishes += i;
        }
        return fishes;
    }
}

fn generate_new_tracker(fishes: &Vec<i64>) -> Tracker {
    let mut new_fishes: Vec<i64> = vec![0; 9]; // Brute force
    for &fish in fishes {
        new_fishes[fish as usize] += 1;
    }
    return Tracker {
        tracker: new_fishes,
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong");
    let fishes: Vec<&str> = contents.split(",").collect();
    let fishes: Vec<i64> = fishes.iter().map(|age| age.parse().unwrap()).collect();

    let mut school: Tracker = generate_new_tracker(&fishes);
    let mut school_clone: Tracker = generate_new_tracker(&fishes);

    for _ in 0..80 {
        school.left_rotate_by_one();
    }

    println!("Part 1: {}", school.get_number_of_fish());

    for _ in 0..256 {
        school_clone.left_rotate_by_one();
    }

    println!("Part 2: {}", school_clone.get_number_of_fish());
}
