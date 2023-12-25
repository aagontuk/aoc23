use std::fs::read_to_string;

fn main() {
    let mut sum: i64 = 0;

    for line in read_lines("input.txt".to_string()) {
        println!("{}", line);
        // iterate over each character
        let mut first: i8 = -1;
        let mut last: i8 = 0;
        for c in line.chars() {
            // check if c is a number
            if c.is_digit(10) {
                // convert c to a number
                last = c.to_digit(10).unwrap() as i8;
                // check if first is empty
                if first == -1 {
                    first = last;
                }
            }
        }
        println!("{}", (first as i64 * 10) + last as i64);
        sum += (first as i64 * 10) + last as i64;
    }

    println!("Sum: {}", sum);
}

fn read_lines(fname: String) -> Vec<String> {
    let contents = read_to_string(fname).expect("Error reading file");
    contents.lines().map(|line| line.to_string()).collect()
}
