use itertools::Itertools;
use num_cpus;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn multithread_brute_force(arr: &Vec<char>, k: usize) -> (usize, usize) {
    println!("\n=== Multithreaded Brute Force Search ===");
    println!("> Starting multithreaded brute force search...");

    // Extract indices of 'G' and 'P'
    let grab_indices: Vec<usize> = arr
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| if c == 'G' { Some(i) } else { None })
        .collect();
    let passenger_indices: Vec<usize> = arr
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| if c == 'P' { Some(i) } else { None })
        .collect();

    println!("  - Grab indices: {:?}", grab_indices);
    println!("  - Passenger indices: {:?}", passenger_indices);

    let num_threads = num_cpus::get();
    println!("> Using {} threads for the search.", num_threads);

    let grab_indices = Arc::new(grab_indices);
    let passenger_indices = Arc::new(passenger_indices);

    let results = Arc::new(Mutex::new(vec![(0, 0); num_threads]));

    let mut handles = vec![];

    for thread_id in 0..num_threads {
        let grab_indices = Arc::clone(&grab_indices);
        let passenger_indices = Arc::clone(&passenger_indices);
        let results = Arc::clone(&results);

        let handle = thread::spawn(move || {
            let mut local_max_matches = 0;
            let mut local_max_solutions = 0;

            let max_r = grab_indices.len().min(passenger_indices.len());

            let chunk_size = (max_r + num_threads - 1) / num_threads;
            let start_r = thread_id * chunk_size + 1;
            let end_r = ((thread_id + 1) * chunk_size).min(max_r);

            println!(
                "  - Thread {}: Processing range r = {} to {}",
                thread_id, start_r, end_r
            );

            for r in (start_r..=end_r).rev() {
                println!(
                    "    - Thread {}: Checking combinations with r = {}",
                    thread_id, r
                );
                for grabs in grab_indices.iter().cloned().combinations(r) {
                    for passengers in passenger_indices.iter().cloned().combinations(r) {
                        if grabs
                            .iter()
                            .zip(passengers.iter())
                            .all(|(&g, &p)| (g as isize - p as isize).abs() as usize <= k)
                        {
                            if r > local_max_matches {
                                println!(
                                    "      - Thread {}: New max match found: {}",
                                    thread_id, r
                                );
                                local_max_matches = r;
                                local_max_solutions = 1;
                            } else if r == local_max_matches {
                                local_max_solutions += 1;
                            }
                        }
                    }
                }

                if local_max_matches == r {
                    break;
                }
            }

            let mut results = results.lock().unwrap();
            results[thread_id] = (local_max_matches, local_max_solutions);
            println!(
                "  - Thread {} finished with local max matches: {} and solutions: {}",
                thread_id, local_max_matches, local_max_solutions
            );
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_results = results.lock().unwrap();
    let max_matches = final_results
        .iter()
        .map(|&(matches, _)| matches)
        .max()
        .unwrap_or(0);
    let total_solutions = final_results
        .iter()
        .filter(|&&(matches, _)| matches == max_matches)
        .map(|&(_, solutions)| solutions)
        .sum();

    println!("> Multithreaded brute force search complete.");
    println!(
        "> Max Matches: {}, Solutions: {}\n",
        max_matches, total_solutions
    );
    (max_matches, total_solutions)
}
