use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let file = fs::File::open("one.txt").unwrap();
    let reader = BufReader::new(file);
    let mut dial = 50;
    let mut zeros = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let action = &line[..1];
        let steps: i32 = line[1..].parse().unwrap();

        let clicks = steps / 100;
        let rotation = steps % 100;

        zeros += clicks;

        match action {
            "R" => {
                if dial + rotation >= 100 {
                    zeros += 1;
                }
                dial = (dial + rotation) % 100;
            }
            "L" => {
                if dial != 0 && dial - rotation <= 0 {
                    zeros += 1;
                }
                dial = dial - rotation;
                dial = ((dial % 100) + 100) % 100;
            }
            _ => (),
        }
    }

    println!("Answer: {zeros}");
}
