use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn nearest_passenger_bipartite_matching(arr: &Vec<char>, k: usize) -> usize {
    println!("\n=== Nearest Passenger Search with Bipartite Matching Style ===");
    println!("> Starting nearest passenger search...");

    let n = arr.len();
    let mut matches = 0;
    let mut passengers = vec![false; n];
    let mut grab_cars = Vec::new();

    // Find all 'G' positions
    for i in 0..n {
        if arr[i] == 'G' {
            grab_cars.push(i);
        }
    }

    // Priority Queue to store potential matches (passengers)
    let mut pq = BinaryHeap::new();

    for &i in &grab_cars {
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
                let distance = (i as isize - j as isize).abs() as usize;
                pq.push((Reverse(distance), j)); // Use distance as the priority
                println!(
                    "    - Available passenger added to priority queue at position {}",
                    j
                );
            }
        }

        // Find the nearest available passenger
        while let Some((Reverse(distance), pos)) = pq.pop() {
            if !passengers[pos] {
                println!(
                    "    - Nearest available passenger found at position {} with distance {}",
                    pos, distance
                );
                matches += 1;
                passengers[pos] = true;
                break; // Stop searching once a match is found
            }
        }
    }

    println!("> Nearest passenger search complete.");
    println!("> Total Matches: {}\n", matches);
    matches
}
