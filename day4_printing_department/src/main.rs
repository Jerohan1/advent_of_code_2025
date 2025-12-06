use std::env::args;
use std::fs::read_to_string;

fn get_neighbours(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<(usize, usize, char)> {
    let mut neighbours = Vec::new();
    let rows = matrix.len();
    let cols = matrix[0].len();

    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let nr = row as isize + dr;
            let nc = col as isize + dc;

            if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                let nr_usize = nr as usize;
                let nc_usize = nc as usize;
                neighbours.push((nr_usize, nc_usize, matrix[nr_usize][nc_usize]));
            }
        }
    }
    return neighbours;
}

fn remove_rolls(matrix: &mut Vec<Vec<char>>) -> (&Vec<Vec<char>>, usize) {
    let mut safe_rolls: Vec<(usize, usize)> = Vec::new();
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            let current_cell = matrix[row][col];
            if current_cell == '@' {
                let mut neighbour_rolls_count = 0;
                let neighbours: Vec<(usize, usize, char)> = get_neighbours(&matrix, row, col);
                for (_, _, ch) in neighbours {
                    if ch == '@' {
                        neighbour_rolls_count += 1;
                    }
                }
                if neighbour_rolls_count < 4 {
                    safe_rolls.push((row, col));
                }
            }
        }
    }

    let n_removed: usize = safe_rolls.len();

    for (row, col) in safe_rolls {
        matrix[row][col] = '.';
    }

    return (matrix, n_removed);
}

fn main() {
    let args: Vec<String> = args().collect();
    let file_path: &String = &args[1];
    let input = read_to_string(file_path).expect("Something went wrong reading the file");
    let lines: Vec<&str> = input.lines().collect();

    let mut roll_matrix: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    //pretty_print_char_matrix(&roll_matrix);
    let mut total_removals = 0;
    let mut removals = 1;
    while removals > 0 {
        removals = remove_rolls(&mut roll_matrix).1;
        total_removals += removals;
    }

    println!("{total_removals}");
}
