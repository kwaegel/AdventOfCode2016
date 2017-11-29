
extern crate regex;
use regex::Regex;

#[allow(dead_code)]
static DAY_1_EXAMPLE: &'static str =
"Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1.";

#[allow(dead_code)]
static DAY_1_INPUT: &'static str =
"Disc #1 has 17 positions; at time=0, it is at position 15.
Disc #2 has 3 positions; at time=0, it is at position 2.
Disc #3 has 19 positions; at time=0, it is at position 4.
Disc #4 has 13 positions; at time=0, it is at position 2.
Disc #5 has 7 positions; at time=0, it is at position 2.
Disc #6 has 5 positions; at time=0, it is at position 0.";

#[derive(Debug)]
struct Disk {
    id: u32,
    num_positions: u32,
    initial_pos: u32
}

fn parse_input(input: &str) -> Vec<Disk> {
    //let re = Regex::new(r"([RL])([:digit:]+)").unwrap();

    let re = Regex::new(r"Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+).").unwrap();;

    let mut results = Vec::new();
    for cap in re.captures_iter(&input) {
        results.push(Disk{
            id: cap[1].parse().unwrap(),
            num_positions: cap[2].parse().unwrap(),
            initial_pos: cap[3].parse().unwrap()
        });
    }
    results
}

// Solve the system of equations for positive T:
// (initial_pos + id + T) % num_positions = 0
fn solve(disks: Vec<Disk>) -> u32 {

    for time in 0..500000 {
        let mut total_residual = 0;
        for disk in &disks {
            let residual = (disk.initial_pos + disk.id + time) % disk.num_positions;
//            println!("  Disk {}: ({} + {} + {}) % {} = {}",
//                     disk.id, disk.initial_pos, disk.id, time, disk.num_positions, residual);
            total_residual += residual;
        }
        if total_residual == 0 {
            return time;
        }
    }
    0
}


fn main() {

    let example_input = parse_input(DAY_1_EXAMPLE);

    println!("{:?}", example_input);

    let day1_sample_time = solve(example_input);
    println!("Part 1 example time: {}", day1_sample_time);

    let day1_time = solve(parse_input(DAY_1_INPUT));
    println!("Part 1: time: {}", day1_time);
    assert_eq!(day1_time, 400589)
}
