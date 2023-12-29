use regex::Regex;

fn main() {
    let mut sum = 0;
    let text = "Game 1: 1 green, 1 blue, 1 red; 3 green, 1 blue, 1 red; 4 green, 3 blue, 1 red; 4 green, 2 blue, 1 red; 3 blue, 3 green";
        
    let re_game = Regex::new(r"Game (\d+)").unwrap();
    let re_colors = Regex::new(r"(\d+) (green|blue|red)").unwrap();

    let parts: Vec<&str> = text.split(':').collect();

    if parts.len() >= 2 {
        let game = parts[0].trim();
        let colors = parts[1].trim();
        let mut game_num = 0;

        for cap in re_game.captures_iter(game) {
            println!("Game: {}", &cap[1]);
            game_num = cap[1].parse::<i32>().unwrap();
        }

        for cap in re_colors.captures_iter(colors) {
            println!("Color: {} - {}", &cap[1], &cap[2]);
            // convert to numbers
            let num = &cap[1].parse::<i32>().unwrap();

            let result = match &cap[2] {
                "green" => if num > &13 {num} else {&-1},
                "blue" => if num > &13 {num} else {&-1},
                "red" => if num > &13 {num} else {&-1},
                _ => &-1
            };

            if result == &-1 {
                sum += game_num;
            }
        }
    }

    println!("Sum: {}", sum);
}
