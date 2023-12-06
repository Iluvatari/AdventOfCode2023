use std::fs;

// const FILE_NAME: &str = "./test.txt";
const FILE_NAME: &str = "./input.txt";

// fn process_input(file_contents: String) -> {

// }

fn get_digit_end_bound(line: &str, start_idx: u32) -> u32 {
    let mut is_digit = true;
    let mut idx: u32 = 0;
    while is_digit {
        if line.len() <= (start_idx + idx) as usize {
            return start_idx + idx - 1;
        }
        is_digit = line
            .chars()
            .nth((start_idx + idx).try_into().unwrap())
            .unwrap()
            .is_digit(10);
        idx += 1;
    }
    return start_idx + idx - 2;
}

fn get_digit_start_end_bounds(line: &str) -> Vec<[u32; 2]> {
    let mut ret_val: Vec<[u32; 2]> = Default::default();
    let mut processing = true;
    let mut idx: u32 = 0;
    while processing {
        if line.chars().nth(idx as usize).unwrap().is_digit(10) {
            let end_bound: u32 = get_digit_end_bound(line, idx);
            ret_val.push([idx, end_bound]);
            idx = end_bound;
        }
        idx += 1;
        processing = idx < line.len().try_into().unwrap();
    }
    return ret_val;
}

fn is_symbol(c: char) -> bool {
    return !(c == '.' || c.is_digit(10));
}

fn has_symbol(file_split: &Vec<&str>, line_num: u32, bounds: [u32; 2]) -> bool {
    // println!("{} - {}", bounds[0], bounds[1]);
    return file_split[(line_num) as usize][bounds[0] as usize..=bounds[1] as usize]
        .chars()
        .any(|c| is_symbol(c));
}

fn has_surrounding_char(file_split: Vec<&str>, line_num: u32, bounds: [u32; 2]) -> bool {
    let is_left = bounds[0] == 0;
    let is_right = bounds[1] == (file_split[line_num as usize].len() - 1) as u32;
    if line_num != 0 {
        // Check top
        let mut bound_tmp = bounds;
        if !is_left {
            bound_tmp[0] -= 1;
        }
        if !is_right {
            bound_tmp[1] += 1;
        }
        if has_symbol(&file_split, line_num - 1, bound_tmp) {
            return true;
        }
    }
    if line_num != (file_split.len() - 1) as u32 {
        // Check botoom
        let mut bound_tmp = bounds;
        if !is_left {
            bound_tmp[0] -= 1;
        }
        if !is_right {
            bound_tmp[1] += 1;
        }
        if has_symbol(&file_split, line_num + 1, bound_tmp) {
            return true;
        }
    }
    if !is_left {
        if is_symbol(
            file_split[line_num as usize]
                .chars()
                .nth((bounds[0] - 1) as usize)
                .unwrap(),
        ) {
            return true;
        }
    }
    if !is_right {
        if is_symbol(
            file_split[line_num as usize]
                .chars()
                .nth((bounds[1] + 1) as usize)
                .unwrap(),
        ) {
            return true;
        }
    }
    return false;
}

fn solve_part_1(file_contents: &String) {
    let file_split: Vec<&str> = file_contents.split("\r\n").collect();
    let mut sum: u32 = 0;
    for (line_num, line) in file_split.iter().enumerate() {
        let bounds = get_digit_start_end_bounds(line);
        for bound in bounds {
            println!("Checking {} - {}", bound[0], bound[1]);
            if has_surrounding_char(file_split.iter().copied().collect(), line_num as u32, bound) {
                let new_var = file_split[line_num as usize][bound[0] as usize..=bound[1] as usize]
                    .parse::<u32>()
                    .unwrap();
                sum += new_var;
                println!("{new_var}");
            }
        }
    }
    println!("{sum}");
}

fn get_digit_start_bound(line: &str, start_idx: u32) -> u32 {
    let mut is_digit = true;
    let mut idx: u32 = 0;
    while is_digit {
        if start_idx - idx == 0 {
            if line
                .chars()
                .nth((0).try_into().unwrap())
                .unwrap()
                .is_digit(10)
            {
                return 0;
            } else {
                return 1;
            }
        }
        is_digit = line
            .chars()
            .nth((start_idx - idx).try_into().unwrap())
            .unwrap()
            .is_digit(10);
        idx += 1;
    }
    return start_idx - idx + 2;
}

fn get_digit_bounds(line: &str, idx: usize) -> [usize; 2] {
    return [
        get_digit_start_bound(line, idx as u32) as usize,
        get_digit_end_bound(line, idx as u32) as usize,
    ];
}

fn get_digits_in_range(line: &str, start_idx: usize, end_idx: usize) -> Vec<u32> {
    let mut ret_val: Vec<u32> = Default::default();
    let mut processing = true;
    let mut idx = start_idx;
    while processing {
        if line.chars().nth(idx).unwrap().is_digit(10) {
            let bounds = get_digit_bounds(line, idx);
            // println!("{}", line);
            // println!("{} - {}", bounds[0], bounds[1]);
            ret_val.push(line[bounds[0]..=bounds[1]].parse::<u32>().unwrap());
            idx = bounds[1];
        }
        idx += 1;
        processing = idx <= end_idx;
    }
    return ret_val;
}

fn get_gear_ratio(file_split: &Vec<&str>, line_num: usize, col_num: usize) -> u32 {
    let mut gear_ratio: u32 = 0;
    // let is_left = col_num == 0;
    // let is_right = col_num == (file_split[line_num].len() - 1);
    let mut bounds: [usize; 2] = Default::default();
    println!("{line_num}");
    println!("{col_num}");
    if col_num < 2 {
        bounds[0] = 0;
    } else {
        bounds[0] = col_num - 1;
    }
    if col_num > file_split[line_num].len() - 3 {
        bounds[1] = file_split[line_num].len() - 1;
    } else {
        bounds[1] = col_num + 1;
    }

    let mut digits: Vec<u32> = Default::default();
    if line_num != 0 {
        let digits_above = get_digits_in_range(file_split[line_num - 1], bounds[0], bounds[1]);
        for digit in digits_above {
            println!("{digit}");
            digits.push(digit);
        }
    }
    if line_num != file_split.len() - 1 {
        let digits_below = get_digits_in_range(file_split[line_num + 1], bounds[0], bounds[1]);
        for digit in digits_below {
            println!("{digit}");
            digits.push(digit);
        }
    }
    if col_num >= 2 {
        let digits_left = get_digits_in_range(file_split[line_num], bounds[0], bounds[0]);
        for digit in digits_left {
            println!("{digit}");
            digits.push(digit);
        }
    }
    if col_num < file_split[line_num].len() - 1 {
        let digits_right = get_digits_in_range(file_split[line_num], bounds[1], bounds[1]);
        for digit in digits_right {
            println!("{digit}");
            digits.push(digit);
        }
    }
    if digits.len() == 2 {
        gear_ratio = digits[0] * digits[1];
    }
    return gear_ratio;
}

fn solve_part_2(file_contents: &String) {
    let file_split: Vec<&str> = file_contents.split("\r\n").collect();
    let mut sum: u32 = 0;
    for (line_num, line) in file_split.iter().enumerate() {
        for (col_num, char) in line.chars().enumerate() {
            if char != '*' {
                continue;
            }
            let gear_ratio = get_gear_ratio(&file_split, line_num, col_num);
            if gear_ratio != 0 {
                println!("{gear_ratio}");
            }
            sum += gear_ratio;
        }
    }
    println!("{sum}");
}

fn main() {
    let file_contents = fs::read_to_string(FILE_NAME).unwrap();
    // process_input(file_contents);
    // solve_part_1(&file_contents);
    solve_part_2(&file_contents);
}
