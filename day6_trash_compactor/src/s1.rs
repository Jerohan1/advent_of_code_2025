use std::env::args;
use std::fs::read_to_string;

fn rotate_counterclockwise(m: Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let rows = m.len();
    let cols = m[0].len();
    let mut out = vec![vec![0; rows]; cols];

    for r in 0..rows {
        for c in 0..cols {
            out[cols - 1 - c][r] = m[r][c];
        }
    }
    out
}

fn main() {
    let args: Vec<String> = args().collect();
    let file_path: &String = &args[1];
    let input = read_to_string(file_path).expect("Something went wrong reading the file");
    let lines: Vec<&str> = input.lines().collect();

    let problems: Vec<Vec<u64>> = lines[..lines.len() - 1]
        .iter()
        .map(|s| {
            s.split_ascii_whitespace()
                .map(|t| t.parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    let problems = rotate_counterclockwise(problems);

    println!("{:?}", problems);
    let operators: Vec<char> = lines[lines.len() - 1]
        .chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .collect();
    println!("{:?}", operators);

    let mut solution: Vec<u64> = Vec::new();

    for i in 0..problems.len() {
        if operators[i] == '+' {
            // sum
            solution.push(problems[i].iter().sum());
        } else if operators[i] == '*' {
            //product
            solution.push(problems[i].iter().product());
        }
    }

    let result: u64 = solution.iter().sum();
    println!("{result}");
}
