use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path: &String = &args[1];

    let input = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let lines: Vec<&str> = input.lines().collect();

    let joltage_size = 12;

    let mut bank_total: Vec<i32> = Vec::new();

    for pack in lines {
        let mut batteries: Vec<i32> = pack
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();

        batteries.sort_by(|a, b| b.cmp(a));

        let mut first = 0;
        let mut second = first + 1;
        let mut best = format!("{}{}", batteries[first], batteries[second])
            .parse::<i32>()
            .unwrap();

        for i in 0..batteries.len() - joltage_size {
            for j in i + 1..batteries.len() {
                if format!("{}{}", batteries[i], batteries[j])
                    .parse::<i32>()
                    .unwrap()
                    > best
                {
                    first = i;
                    second = j;
                    best = format!("{}{}", batteries[first], batteries[second])
                        .parse::<i32>()
                        .unwrap();
                }
            }
        }

        println!("{best}");
        bank_total.push(best);
    }
    let total_output: i32 = bank_total.iter().sum();
    println!("{total_output}");
}
