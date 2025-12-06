use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path: &String = &args[1];

    let input = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    //let input = "16-33";

    let id_ranges: Vec<&str> = input.split(",").collect();

    let mut invalid_ids: u64 = 0;

    for range in id_ranges {
        let interval: Vec<&str> = range.split("-").collect();
        let start: u64 = interval[0].parse().expect("Not a number");
        let end: u64 = interval[1].parse().expect("Not a number");

        let ids: Vec<String> = (start..=end).map(|n| n.to_string()).collect();

        for id in ids {
            let len = id.len();

            for i in 1..=len / 2 {
                if len % i != 0 {
                    continue;
                }

                let pattern = &id[0..i];

                if pattern.repeat(len / i) == id {
                    //println!("{id}");
                    invalid_ids += id.parse::<u64>().expect("Not a number");
                    break;
                }
            }
        }
    }

    println!("Sum of invalid ids: {invalid_ids}");
}
