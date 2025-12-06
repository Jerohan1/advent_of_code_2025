use std::io::{self, Read};

fn main() {
    let mut dial: i32 = 50;
    let mut pass: i32 = 0;
    let mut input: String = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read line");

    let lines: Vec<&str> = input.lines().collect();

    println!("{:?}", lines);

    for movement in lines {
        let direction: &str = &movement[0..1];
        let distance: &i32 = &movement[1..].parse().expect("Not a number");

        let turns: i32 = distance / 100;
        pass += turns;
        let travel: i32 = distance % 100;

        if direction == "R" {
            if dial + travel <= 99 {
                dial = dial + travel;
            } else {
                dial = dial + travel - 100;
                if dial != 0 {
                    pass += 1;
                }
            }
        }

        if direction == "L" {
            if dial - travel >= 0 {
                dial = dial - travel;
            } else {
                if dial != 0 {
                    pass += 1;
                }
                dial = dial - travel + 100;
            }
        }

        if dial == 0 {
            pass += 1;
        }

        println!("Direction: {direction}, distance {turns} {travel}, dial {dial}, pass {pass}");
    }
    println!("{pass}");
}
