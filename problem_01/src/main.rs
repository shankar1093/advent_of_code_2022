use std::env;
use std::fs;

fn top_three(mut total_calories: Vec<i32>) {
    total_calories.sort();
    let sum_three :i32= total_calories.iter().rev().take(3).sum();
    println!("{:?}",sum_three);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    let mut sol = Vec::new();
    let mut temp = Vec::new();
    let lines = contents.lines();
    for line in lines {
        if line.chars().count() == 0{
            let sum: i32 = temp.iter().sum();
            sol.push(sum);
            temp.clear()
        }
        else {
            let calorie: i32 = line.parse().unwrap();
            temp.push(calorie);
        }
    }
    let max_value = *sol.iter().max().unwrap();
    top_three(sol);
}