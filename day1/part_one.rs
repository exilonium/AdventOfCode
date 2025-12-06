use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let file = fs::File::open("one.txt").unwrap();
    let reader = BufReader::new(file);
    let mut counter = 50; // problem statement that counter starts at 50
    let mut res = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let action = &line[..1];
        let steps: i32 = line[1..].parse().unwrap();
        //println!("{action}");
        match action {
            "L" => counter -= steps,
            "R" => counter += steps,
            _ => (),
        }
        //counter = ((counter % 100) + 100) % 100; the below is same at this one
        // we can also do counter +=100000 and then modulus but either way this program works
        counter = counter.rem_euclid(100);
        if counter == 0 {
            res += 1;
        }
        //println!("{line} {res}");
    }
    println!("Answer : {res}");
}

// the answer to one.txt was 1040
