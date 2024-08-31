pub fn nearest_passenger_greedy(arr: &Vec<char>, k: usize) -> usize {
    println!("\n=== Greedy Nearest Passenger Search ===");
    println!("> Starting greedy nearest passenger search...");

    let n = arr.len();
    let mut matches = 0;
    let mut passengers = vec![false; n];

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

            // Search within the defined range for the nearest passenger
            let mut found = false;
            for j in start..=end {
                if arr[j] == 'P' && !passengers[j] {
                    println!("    - Found available passenger at position {}", j);
                    matches += 1;
                    passengers[j] = true;
                    found = true;
                    break; // Stop searching once a match is found
                }
            }

            if !found {
                println!("    - No available passengers in range.");
            }
        }
    }

    println!("> Greedy nearest passenger search complete.");
    println!("> Total Matches: {}\n", matches);
    matches
}
