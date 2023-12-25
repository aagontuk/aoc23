use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let mut sum: i64 = 0;

    let str_nums = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut first_occ = HashMap::new();
    let mut last_occ = HashMap::new();

    for s in str_nums.iter() {
        first_occ.insert(s.to_string(), -1);
    }

    for s in str_nums.iter() {
        last_occ.insert(s.to_string(), -1);
    }

    for line in read_lines("input.txt".to_string()) {
        println!("{}", line);
        
        for s in str_nums.iter() {
            first_occ.insert(s.to_string(), -1);
        }

        for s in str_nums.iter() {
            last_occ.insert(s.to_string(), -1);
        }

        for s in str_nums.iter() {
            let i = first_occurrence(&line, s.to_string());
            if i != -1 {
                first_occ.insert(s.to_string(), i);
            }
        }
        
        for s in str_nums.iter() {
            let i = last_occurrence(&line, s.to_string());
            if i != -1 {
                last_occ.insert(s.to_string(), i);
            }
        }

        // iterate over each character
        let mut first: i8 = -1;
        let mut first_pos: i8 = 0;
        let mut last: i8 = 0;
        let mut last_pos: i8 = 0;
        for (i, c) in line.chars().enumerate() {
            // check if c is a number
            if c.is_digit(10) {
                // convert c to a number
                last = c.to_digit(10).unwrap() as i8;
                last_pos = i as i8;
                // check if first is empty
                if first == -1 {
                    first = last;
                    first_pos = i as i8;
                }
            }
        }

        let mut final_first: i8 = first;
        let mut final_last: i8 = last;

        //println!("{}: {}", first, first_pos);
        //println!("{}: {}", last, last_pos);
        //println!("{:?}", first_occ);
        //println!("{:?}", last_occ);

        for (key, val) in first_occ.iter() {
            if (val < &first_pos) && (val != &-1) {
                final_first = string_to_num(key);
                first_pos = *val;
            }
        }

        for (key, val) in last_occ.iter() {
            if (val > &last_pos) && (val != &-1) {
                final_last = string_to_num(key);
                last_pos = *val;
            }
        }
        
        println!("{}", (final_first as i64 * 10) + final_last as i64);
        sum += (final_first as i64 * 10) + final_last as i64;
    }

    println!("Sum: {}", sum);
}

fn read_lines(fname: String) -> Vec<String> {
    let contents = read_to_string(fname).expect("Error reading file");
    contents.lines().map(|line| line.to_string()).collect()
}

fn first_occurrence(s: &String, n: String) -> i8 {
    match s.find(&n) {
        Some(i) => i as i8,
        None => -1,
    }
}

fn last_occurrence(s: &String, n: String) -> i8 {
    match s.rfind(&n) {
        Some(i) => i as i8,
        None => -1,
    }
}

fn string_to_num(s: &String) -> i8 {
    match s.as_str() {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => -1,
    }
}
