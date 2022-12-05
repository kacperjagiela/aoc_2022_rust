use std::env;
use std::fs;

const A: char = 'A';
const B: char = 'B';

const X: char = 'X';
const Y: char = 'Y';

fn main() {
    // read file
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let file_lines = contents.lines();

    let shapes: Vec<usize> = vec![1, 2, 3];
    let win_points: Vec<usize> = vec![0, 3, 6];

    let mut score = 0;

    for line in file_lines {
        let first_char = line.chars().next().unwrap();
        let last_char = line.chars().last().unwrap();

        score += compare_win(first_char, last_char, &shapes, &win_points);
    }

    println!("{}", score);
}

fn compare_win(first: char, second: char, shapes: &Vec<usize>, win_points: &Vec<usize>) -> usize {
    let win_index = get_index_for_win(second);
    let shape_index = get_index_for_shape(first, win_index);

    return &shapes[shape_index] + &win_points[win_index];
}

fn get_index_for_shape(shape: char, win_index: usize) -> usize {
    if shape == A {
        if win_index == 0 {
            return 2;
        } else if win_index == 1 {
            return 0;
        } else {
            return 1;
        }
    } else if shape == B {
        if win_index == 0 {
            return 0;
        } else if win_index == 1 {
            return 1;
        } else {
            return 2;
        }
    } else {
        if win_index == 0 {
            return 1;
        } else if win_index == 1 {
            return 2;
        } else {
            return 0;
        }
    }
}

fn get_index_for_win(win: char) -> usize {
    if win == X {
        return 0;
    } else if win == Y {
        return 1;
    }

    return 2;
}
