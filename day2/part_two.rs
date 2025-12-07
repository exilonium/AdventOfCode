use std::fs;

// this is the most inefficient solution but it works part 2

fn is_valid_id(s: &str) -> bool {
    let len = s.len();

    // Try all possible pattern lengths from 1 to len/2
    for pattern_len in 1..=(len / 2) {
        // Only check if the pattern length divides the string length evenly
        if len % pattern_len == 0 {
            let repeats = len / pattern_len;

            // Must repeat at least twice
            if repeats >= 2 {
                let pattern = &s[0..pattern_len];

                // Check if entire string is this pattern repeated
                if s == pattern.repeat(repeats) {
                    return false;
                }
            }
        }
    }

    true // Valid - no repeating pattern found
}

fn main() {
    // reading the file
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
