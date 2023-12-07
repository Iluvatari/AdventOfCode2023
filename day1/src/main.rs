use std::fs;

// const file_path: &str = "./test.txt";
// const file_path: &str = "./test2.txt";
// const file_path: &str = "./input.txt";
// const file_path: &str = "./test_redo.txt";
const file_path: &str = "./input_redo.txt";

const STRING_WORDS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn is_digit(string_in: String) -> u32 {
    if string_in.chars().nth(0).expect("not a string").is_digit(10) {
        return string_in
            .chars()
            .nth(0)
            .expect("not a string")
            .to_digit(10)
            .expect("msg");
    }

    for (idx, string_word) in STRING_WORDS.iter().enumerate() {
        if string_in.starts_with(string_word) {
            return idx.try_into().unwrap();
        }
    }
    return 0;
}

fn solve_part_1(split_contents: &Vec<&str>) {
    let mut cal_vals: Vec<String> = Vec::new();
    let mut last_digit: char = ';';
    for (_idx, line) in split_contents.iter().enumerate() {
        let mut has_first = false;
        for char in line.chars() {
            if char.is_digit(10) {
                if !has_first {
                    cal_vals.push(char.to_string());
                    has_first = true;
                }
                last_digit = char;
            }
        }
        cal_vals.last_mut().expect("no last").push(last_digit);
    }
    let mut sum = 0;
    for line in cal_vals {
        // println!("{line}");
        sum += line.parse::<i32>().unwrap();
    }
    println!("{sum}");
}
fn solve_part_2(split_contents: &Vec<&str>) {
    let mut cal_vals: Vec<String> = Vec::new();
    for (_idx, line) in split_contents.iter().enumerate() {
        let mut has_first = false;
        let mut last_digit: char = ';';
        for idx in 0..line.len() {
            let digit = is_digit(line.chars().skip(idx).collect());
            if digit > 0 {
                if !has_first {
                    cal_vals.push(digit.to_string());
                    has_first = true;
                }
                last_digit = digit.to_string().chars().nth(0).unwrap();
            }
        }
        cal_vals.last_mut().expect("no last").push(last_digit);
    }
    let mut sum = 0;
    for line in cal_vals {
        // println!("{line}");
        sum += line.parse::<i32>().unwrap();
    }
    println!("{sum}");
}

fn main() {
    let contents = fs::read_to_string(file_path).expect("Failed to read file {file_path}");
    let split_contents: Vec<&str> = contents.split("\r\n").collect();
    solve_part_1(&split_contents);
    solve_part_2(&split_contents);
}
