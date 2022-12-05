use std::env;
use std::fs;

fn main() {
    // read file
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let file_lines = contents.lines();

    let mut elves_vec: Vec<i32> = Vec::new();

    let mut current_calories: i32 = 0;

    for calorie in file_lines {
        if calorie == "" {
            elves_vec.push(current_calories);
            current_calories = 0;
        } else {
            current_calories += calorie.parse::<i32>().unwrap();
        }
    }

    println!("{}", sum_of_top_elves(elves_vec, 3));
}

fn sum_of_top_elves(mut elves_vec: Vec<i32>, top_to_sum: usize) -> i32 {
    elves_vec.sort();
    elves_vec.reverse();

    let mut sum = 0;
    let slice = elves_vec.as_slice();

    for i in slice[0..top_to_sum].iter() {
        sum += i
    }

    return sum;
}
