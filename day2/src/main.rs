use std::fs;
// const FILE_NAME: &str = "./test.txt";
// const FILE_NAME: &str = "./input.txt";
const FILE_NAME: &str = "./input_redo.txt";
const MAX_BLUE: u32 = 14;
const MAX_GREEN: u32 = 13;
const MAX_RED: u32 = 12;

struct Game {
    id: u32,
    red: Vec<u32>,
    blue: Vec<u32>,
    green: Vec<u32>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            id: Default::default(),
            red: Default::default(),
            blue: Default::default(),
            green: Default::default(),
        }
    }
}

fn get_nums_before_color(line: &str, color: &str) -> Vec<u32> {
    let mut ret_val: Vec<u32> = Default::default();
    let color_splits: Vec<&str> = line.split(color).collect();
    let color_split_count = color_splits.len();
    for (idx, color_split) in color_splits.iter().enumerate() {
        if idx == color_split_count - 1 {
            continue;
        }
        let num_maybe = color_split
            .split(' ')
            .nth_back(1)
            .unwrap_or("")
            .parse::<u32>();
        if num_maybe.is_ok() {
            ret_val.push(num_maybe.unwrap());
        }
    }
    return ret_val;
}

fn process_input(file_contents: String) -> Vec<Game> {
    let file_lines: Vec<&str> = file_contents.split("\r\n").collect();
    let mut games: Vec<Game> = Vec::new();
    for line in file_lines {
        let mut game_tmp: Game = Default::default();
        game_tmp.id = line
            .split(":")
            .nth(0)
            .unwrap()
            .split(" ")
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();
        game_tmp.blue = get_nums_before_color(line, "blue");
        game_tmp.red = get_nums_before_color(line, "red");
        game_tmp.green = get_nums_before_color(line, "green");

        games.push(game_tmp);
    }
    return games;
}

fn solve_part_one(games: &Vec<Game>) {
    let mut sum: u64 = 0;
    for game in games {
        if (*game.blue.iter().max().unwrap_or(&0) > MAX_BLUE)
            || (*game.green.iter().max().unwrap_or(&0) > MAX_GREEN)
            || (*game.red.iter().max().unwrap_or(&0) > MAX_RED)
        {
        } else {
            sum += game.id as u64;
        }
    }
    println!("{sum}");
}

fn solve_part_two(games: &Vec<Game>) {
    let mut sum: u64 = 0;
    for game in games {
        let max_blue: u64 = *game.blue.iter().max().unwrap_or(&0) as u64;
        let max_green: u64 = *game.green.iter().max().unwrap_or(&0) as u64;
        let max_red: u64 = *game.red.iter().max().unwrap_or(&0) as u64;
        sum += max_blue * max_green * max_red;
    }
    println!("{sum}");
}

fn main() {
    let file_contents = fs::read_to_string(FILE_NAME).unwrap();
    let games = process_input(file_contents);
    solve_part_one(&games);
    solve_part_two(&games);
}
