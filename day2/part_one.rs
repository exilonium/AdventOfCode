use std::fs;

// this is the most inefficient solution but it works

fn is_valid_id(s: &str) -> bool {
    let len = s.len();

    // String must have even length to be split in half
    if len % 2 != 0 {
        return true; // Odd length can't be repeated exactly twice
    }

    let half = len / 2;
    let first_half = &s[0..half];
    let second_half = &s[half..];

    first_half != second_half // Valid if halves are different
}

fn main() {
    //reading the file
    let content = fs::read_to_string("file.txt").unwrap();

    // converting the string into vector
    let myrange: Vec<&str> = content.strip_suffix("\n").unwrap().split(",").collect();

    let mut ans = 0;

    for i in myrange {
        // converting the number into the range by splitting it into two chunks
        let parts: Vec<&str> = i.split('-').collect();
        let start: i64 = parts[0].parse().unwrap();
        let end: i64 = parts[1].parse().unwrap();

        // iterating the range
        for i in start..=end {
            let num = i.to_string();

            if !is_valid_id(&num) {
                ans += i;
                // println!("{start}-{end}-->{i}");
            }
        }
    }
    println!("{ans}");
}
// the answer was 18595663903
