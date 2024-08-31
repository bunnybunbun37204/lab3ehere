use std::fs;

pub fn read_input_from_file(file_name: &str) -> (Vec<char>, usize) {
    println!("\n=== Reading Input from File ===");
    println!("> Reading input from file: {}", file_name);
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let mut lines = contents.lines();
    let arr: Vec<char> = lines.next().unwrap().chars().collect();
    let k: usize = lines.next().unwrap().parse().expect("Invalid number");
    println!("> Input array: {:?}", arr);
    println!("> Max distance k: {}\n", k);
    (arr, k)
}
