use itertools::Itertools;

pub fn brute_force(arr: &Vec<char>, k: usize) -> (usize, usize) {
    println!("\n=== Brute Force Search ===");
    println!("> Starting brute force search...");

    let mut max_matches = 0;
    let mut max_solutions = 0;

    // Finding indices of 'G' and 'P'
    println!("> Finding all Grab and Passenger indices...");
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

    // Iterating over all possible combinations
    let min_length = grab_indices.len().min(passenger_indices.len());
    println!("> Iterating over combinations from 1 to {}", min_length);

    for r in (1..=min_length).rev() {
        println!("  - Checking combinations with r = {}", r);
        for grabs in grab_indices.iter().cloned().combinations(r) {
            for passengers in passenger_indices.iter().cloned().combinations(r) {
                // Check if all combinations are within distance k
                if grabs
                    .iter()
                    .zip(passengers.iter())
                    .all(|(&g, &p)| (g as isize - p as isize).abs() as usize <= k)
                {
                    if r > max_matches {
                        println!("    - New max match found: {}", r);
                        max_matches = r;
                        max_solutions = 1;
                    } else if r == max_matches {
                        max_solutions += 1;
                    }
                }
            }
        }
    }

    println!("> Brute force search complete.");
    println!(
        "> Max Matches: {}, Solutions: {}\n",
        max_matches, max_solutions
    );
    (max_matches, max_solutions)
}
