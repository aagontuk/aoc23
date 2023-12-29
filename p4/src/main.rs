use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let mut sum: i64 = 0;
    
    let re_colors = Regex::new(r"(\d+) (green|blue|red)").unwrap();

    for line in read_lines("input.txt".to_string()) {
        let parts: Vec<&str> = line.split(':').collect();
        let mut max_red = -1;
        let mut max_blue = -1;
        let mut max_green = -1;

        if parts.len() >= 2 {
            let colors = parts[1].trim();

            for cap in re_colors.captures_iter(colors) {
                let num = cap[1].parse::<i64>().unwrap();

                match &cap[2] {
                    "green" => if num > max_green {max_green = num},
                    "blue" => if num > max_blue {max_blue = num},
                    "red" => if num > max_red {max_red = num},
                    _ => (),
                };

            }
        }

        sum += max_red * max_green * max_blue;
    }

    println!("Sum: {}", sum);
}

fn read_lines(fname: String) -> Vec<String> {
    let contents = read_to_string(fname).expect("Error reading file");
    contents.lines().map(|line| line.to_string()).collect()
}
