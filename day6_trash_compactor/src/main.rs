use std::env::args;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = args().collect();
    let file_path: &String = &args[1];
    let input = read_to_string(file_path).expect("Something went wrong reading the file");
    let lines: Vec<&str> = input.lines().collect();

    let mut results: Vec<u64> = Vec::new();
    let mut cur_number: String = String::new();
    let mut cur_problem: Vec<u64> = Vec::new();

    for y in (0..lines[0].len()).rev() {
        for x in 0..lines.len() {
            let cur_char = lines[x].chars().nth(y).unwrap();

            if cur_char.is_numeric() {
                cur_number.push(cur_char);
            } else if cur_char == ' ' {
                if !cur_number.is_empty() {
                    cur_problem.push(cur_number.parse::<u64>().unwrap());
                    cur_number.clear();
                }
            } else if cur_char == '+' || cur_char == '*' {
                if !cur_number.is_empty() {
                    cur_problem.push(cur_number.parse::<u64>().unwrap());
                    cur_number.clear();
                }

                let solution: u64 = match cur_char {
                    '+' => cur_problem.iter().sum(),
                    '*' => cur_problem.iter().product(),
                    _ => unreachable!(),
                };
                results.push(solution);
                cur_problem.clear();
            }
        }
    }
    let output: u64 = results.iter().sum();
    println!("{output}");
}
