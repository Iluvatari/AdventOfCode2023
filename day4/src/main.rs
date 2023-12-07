use std::fs;

// const FILE_NAME: &str = "./test.txt";
const FILE_NAME: &str = "./input.txt";

fn process_input(file_contents: &Vec<&str>) -> Vec<[Vec<u32>; 2]> {
    let mut ret_val: Vec<[Vec<u32>; 2]> = Default::default();
    for line in file_contents {
        let mut tmp: [Vec<u32>; 2] = Default::default();
        let split_line: Vec<&str> = line.split(" | ").collect();
        let numbers: Vec<&str> = split_line
            .first()
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .split_ascii_whitespace()
            .collect();
        for number in numbers {
            tmp[0].push(number.parse::<u32>().unwrap());
        }
        let numbers1: Vec<&str> = split_line
            .last()
            .unwrap()
            .split_ascii_whitespace()
            .collect();
        for number in numbers1 {
            tmp[1].push(number.parse::<u32>().unwrap());
        }
        ret_val.push(tmp);
    }
    return ret_val;
}

fn solve_part_1(file_contents: &Vec<[Vec<u32>; 2]>) {
    let mut sum: u32 = 0;
    for line in file_contents {
        let mut num_winning: u32 = 0;
        for winning_number in &line[0] {
            if line[1].contains(winning_number) {
                num_winning += 1;
            }
        }
        if num_winning > 0 {
            let points = 1 << (num_winning - 1);
            println!("{points}");
            sum += points;
        }
    }
    println!("{sum}");
}

fn solve_part_2(file_contents: &Vec<[Vec<u32>; 2]>) {
    let mut num_winners: Vec<u32> = Default::default();
    let mut copy_count: Vec<u32> = Default::default();
    for line in file_contents {
        num_winners.push(0);
        copy_count.push(0);
        for (idx, winning_number) in (&line[0]).iter().enumerate() {
            if line[1].contains(winning_number) {
                *num_winners.last_mut().unwrap() += 1;
            }
        }
    }
    for (idx, &winners) in num_winners.iter().enumerate() {
        copy_count[idx] += 1; // The original
        for in_idx in 1..=winners {
            copy_count[idx + in_idx as usize] += copy_count[idx];
        }
    }
    let mut sum: u32 = 0;
    for count in copy_count {
        sum += count;
    }
    println!("{sum}");
}

fn main() {
    let file_contents_tmp = fs::read_to_string(FILE_NAME).unwrap();
    let file_contents: Vec<&str> = file_contents_tmp.split("\r\n").collect();
    let input = process_input(&file_contents);
    solve_part_1(&input);
    solve_part_2(&input);
}
