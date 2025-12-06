use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    let input = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let lines: Vec<&str> = input.lines().collect();

    let joltage_size = 12;

    let mut joltage: Vec<i64> = Vec::new();

    for pack in lines {
        let batteries: Vec<i64> = pack
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect();
        let mut activated: Vec<String> = Vec::new();

        let mut next_index = 0;

        for i in (0..joltage_size).rev() {
            let slice = &batteries[next_index..batteries.len() - i];
            if slice.is_empty() {
                break;
            }

            let (index, val) =
                slice
                    .iter()
                    .enumerate()
                    .fold((0, &slice[0]), |(idx_max, val_max), (idx, val)| {
                        if *val > *val_max {
                            (idx, val)
                        } else {
                            (idx_max, val_max)
                        }
                    });
            //println!("{:?}", slice);
            activated.push(val.to_string());
            next_index += index + 1;
        }
        //println!("{:?}", activated);
        let bank_nr: i64 = activated.join("").parse::<i64>().unwrap();
        //println!("{bank_nr}");
        joltage.push(bank_nr);
    }
    let total_output: i64 = joltage.iter().sum();
    println!("{total_output}");
    return;
}
