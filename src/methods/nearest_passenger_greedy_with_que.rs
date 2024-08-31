use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn nearest_passenger_greedy_priority_queue(arr: &Vec<char>, k: usize) -> usize {
    println!("\n=== Greedy Nearest Passenger Search with Priority Queue ===");
    println!("> Starting greedy nearest passenger search...");

    let n = arr.len();
    let mut matches = 0;
    let mut passengers = vec![false; n];

    // Priority Queue to store available passengers within range
    let mut pq = BinaryHeap::new();

    for i in 0..n {
        if arr[i] == 'G' {
            println!("> Checking Grab car at position {}", i);

            // Define the range to search for passengers
            let start = (i as isize - k as isize).max(0) as usize;
            let end = (i + k).min(n - 1);
            println!(
                "  - Searching for passengers in range: {} to {}",
                start, end
            );

            // Add all available passengers in the range to the priority queue
            for j in start..=end {
                if arr[j] == 'P' && !passengers[j] {
                    pq.push(Reverse(j)); // Reverse to get the smallest index (nearest passenger)
                    println!("    - Available passenger added to priority queue at position {}", j);
                }
            }

            // Find the nearest available passenger
            while let Some(Reverse(pos)) = pq.pop() {
                if !passengers[pos] {
                    println!("    - Nearest available passenger found at position {}", pos);
                    matches += 1;
                    passengers[pos] = true;
                    break; // Stop searching once a match is found
                }
            }

            if passengers.iter().all(|&p| !p) {
                println!("    - No available passengers in range.");
            }
        }
    }

    println!("> Greedy nearest passenger search complete.");
    println!("> Total Matches: {}\n", matches);
    matches
}
