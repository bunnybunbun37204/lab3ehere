pub mod brute_force;
pub mod efficient_search;
pub mod multithread_brute_force;
pub mod nearest_passenger_greedy;

use std::time::Instant;

pub fn run_brute_force(arr: &Vec<char>, k: usize) {
    let start_time = Instant::now();
    let (max_passengers, solutions) = brute_force::brute_force(arr, k);
    let duration = start_time.elapsed();
    println!("Brute Force Result:");
    println!("  - Maximum Passengers: {}", max_passengers);
    println!("  - Total Solutions: {}", solutions);
    println!("  - Runtime: {:?}\n", duration);
}

pub fn run_multithread_brute_force(arr: &Vec<char>, k: usize) {
    let start_time = Instant::now();
    let (max_passengers, solutions) = multithread_brute_force::multithread_brute_force(arr, k);
    let duration = start_time.elapsed();
    println!("Multithreaded Brute Force Result:");
    println!("  - Maximum Passengers: {}", max_passengers);
    println!("  - Total Solutions: {}", solutions);
    println!("  - Runtime: {:?}\n", duration);
}

pub fn run_nearest_passenger_greedy(arr: &Vec<char>, k: usize) {
    let start_time = Instant::now();
    let max_passengers = nearest_passenger_greedy::nearest_passenger_greedy(arr, k);
    let duration = start_time.elapsed();
    println!("Nearest Passenger Greedy Result:");
    println!("  - Maximum Passengers: {}", max_passengers);
    println!("  - Runtime: {:?}\n", duration);
}

pub fn run_efficient_search(arr: &Vec<char>, k: usize) {
    let start_time = Instant::now();
    let (max_passengers, _) = efficient_search::efficient_search(arr, k);
    let duration = start_time.elapsed();
    println!("Efficient Search Result:");
    println!("  - Maximum Passengers: {}", max_passengers);
    println!("  - Runtime: {:?}\n", duration);
}
