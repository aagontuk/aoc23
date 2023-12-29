use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let mut sum = 0;
    
    let re_game = Regex::new(r"Game (\d+)").unwrap();
    let re_colors = Regex::new(r"(\d+) (green|blue|red)").unwrap();

    for line in read_lines("input.txt".to_string()) {
        let parts: Vec<&str> = line.split(':').collect();
        let mut is_possible = true;
        let mut game_num = 0;

        if parts.len() >= 2 {
            let game = parts[0].trim();
            let colors = parts[1].trim();

            for cap in re_game.captures_iter(game) {
                game_num = cap[1].parse::<i32>().unwrap();
            }

            for cap in re_colors.captures_iter(colors) {
                let num = cap[1].parse::<i32>().unwrap();

                let result = match &cap[2] {
                    "green" => if num <= 13 {num} else {-1},
                    "blue" => if num <= 14 {num} else {-1},
                    "red" => if num <= 12 {num} else {-1},
                    _ => -1
                };

                if result == -1 {
                    is_possible = false;
                }
            }
        }

        if is_possible {
            sum += game_num;
        }
    }

    println!("Sum: {}", sum);
}

fn read_lines(fname: String) -> Vec<String> {
    let contents = read_to_string(fname).expect("Error reading file");
    contents.lines().map(|line| line.to_string()).collect()
}
