mod input;
mod methods;
mod plot;

use input::read_input_from_file;
use plot::{plot_durations, DurationResult};
use std::collections::HashSet;

fn main() {
    // Get the input file name from the command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run <input_file>");
        std::process::exit(1);
    }

    let file_name = &args[1];
    let (arr, k) = read_input_from_file(file_name);

    println!("Select options (comma-separated):");
    println!("1. Run Brute Force Method");
    println!("2. Run Multithreaded Brute Force Method");
    println!("3. Run Nearest Passenger Greedy Method");
    println!("4. Run Nearest Passenger Greedy With Que Method");
    println!("5. Run Efficient Search Method");
    println!("6. Run Maximum bipartite Method");
    println!("7. Run All Methods");
    println!("Enter your choices:");

    let mut choices = String::new();
    std::io::stdin()
        .read_line(&mut choices)
        .expect("Failed to read input");

    // Parse and validate the user input
    let choices: HashSet<usize> = choices
        .trim()
        .split(',')
        .filter_map(|s| s.trim().parse::<usize>().ok())
        .collect();

    if choices.is_empty() {
        println!("No valid choices entered.");
        return;
    }

    let mut results = Vec::new();

    // Run the selected methods and collect results
    for choice in choices {
        match choice {
            1 => {
                let start_time = std::time::Instant::now();
                methods::run_brute_force(&arr, k);
                let duration = start_time.elapsed();
                results.push(DurationResult {
                    name: "Brute Force".to_string(),
                    duration,
                });
            }
            2 => {
                let start_time = std::time::Instant::now();
                methods::run_multithread_brute_force(&arr, k);
                let duration = start_time.elapsed();
                results.push(DurationResult {
                    name: "Multithreaded Brute Force".to_string(),
                    duration,
                });
            }
            3 => {
                let start_time = std::time::Instant::now();
                methods::run_nearest_passenger_greedy(&arr, k);
                let duration = start_time.elapsed();
                results.push(DurationResult {
                    name: "Nearest Passenger Greedy".to_string(),
                    duration,
                });
            }
            4 => {
                let start_time = std::time::Instant::now();
                methods::run_nearest_passenger_greedy_with_priority_que(&arr, k);
                let duration = start_time.elapsed();
                results.push(DurationResult {
                    name: "Nearest Passenger Greedy With Que".to_string(),
                    duration,
                });
            }
            5 => {
                let start_time = std::time::Instant::now();
                methods::run_efficient_search(&arr, k);
                let duration = start_time.elapsed();
                results.push(DurationResult {
                    name: "Efficient Search".to_string(),
                    duration,
                });
            }
            6 => {
                let start_time = std::time::Instant::now();
                methods::run_maximum_birpartite(&arr, k);
                let duration = start_time.elapsed();
                results.push(DurationResult {
                    name: "Maximum Biparitie".to_string(),
                    duration,
                });
            }
            7 => {
                let start_time = std::time::Instant::now();
                methods::run_brute_force(&arr, k);
                let duration = start_time.elapsed();
                results.push(DurationResult {
                    name: "Brute Force".to_string(),
                    duration,
                });

                let start_time = std::time::Instant::now();
                methods::run_multithread_brute_force(&arr, k);
                let duration = start_time.elapsed();
                results.push(DurationResult {
                    name: "Multithreaded Brute Force".to_string(),
                    duration,
                });

                let start_time = std::time::Instant::now();
                methods::run_nearest_passenger_greedy(&arr, k);
                let duration = start_time.elapsed();
                results.push(DurationResult {
                    name: "Nearest Passenger Greedy".to_string(),
                    duration,
                });

                let start_time = std::time::Instant::now();
                methods::run_efficient_search(&arr, k);
                let duration = start_time.elapsed();
                results.push(DurationResult {
                    name: "Efficient Search".to_string(),
                    duration,
                });
            }
            _ => println!("Invalid choice: {}", choice),
        }
    }

    // Plot the durations
    if let Err(e) = plot_durations(&results) {
        eprintln!("Failed to generate plot: {}", e);
    }
}
