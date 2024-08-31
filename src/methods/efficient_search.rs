pub fn efficient_search(arr: &Vec<char>, k: usize) -> (usize, usize) {
    println!("\n=== Efficient Search ===");
    println!("> Starting efficient search...");

    // Extract indices of 'G' and 'P'
    let mut grab_indices: Vec<usize> = arr
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| if c == 'G' { Some(i) } else { None })
        .collect();
    let mut passenger_indices: Vec<usize> = arr
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| if c == 'P' { Some(i) } else { None })
        .collect();

    // Print indices before sorting
    println!("  - Grab indices (before sort): {:?}", grab_indices);
    println!(
        "  - Passenger indices (before sort): {:?}",
        passenger_indices
    );

    // Sort the indices
    grab_indices.sort();
    passenger_indices.sort();

    // Print indices after sorting
    println!("  - Grab indices (after sort): {:?}", grab_indices);
    println!(
        "  - Passenger indices (after sort): {:?}",
        passenger_indices
    );

    let mut max_matches = 0;
    let mut total_solutions = 0;

    let mut i = 0; // Pointer for grab_indices
    let mut j = 0; // Pointer for passenger_indices

    // Iterate through both lists to find matching ranges
    while i < grab_indices.len() && j < passenger_indices.len() {
        let grab = grab_indices[i];
        let passenger = passenger_indices[j];
        let diff = (grab as isize - passenger as isize).abs() as usize;

        println!(
            "  - Comparing Grab index {} (value {}) with Passenger index {} (value {})",
            grab, grab_indices[i], passenger, passenger_indices[j]
        );
        println!("    Difference = {}", diff);

        if diff <= k {
            println!("    Within range (<= {})", k);
            let mut current_matches = 0;

            // Count matches within the range
            while i < grab_indices.len()
                && j < passenger_indices.len()
                && (grab_indices[i] as isize - passenger_indices[j] as isize).abs() as usize <= k
            {
                current_matches += 1;
                println!(
                    "    - Matching Grab index {} (value {}) with Passenger index {} (value {})",
                    grab_indices[i], grab_indices[i], passenger_indices[j], passenger_indices[j]
                );
                i += 1;
                j += 1;
            }

            // Update max_matches and total_solutions based on current_matches
            if current_matches > max_matches {
                max_matches = current_matches;
                total_solutions = 1;
                println!("    New max match: {}", max_matches);
            } else if current_matches == max_matches {
                total_solutions += 1;
                println!(
                    "    Incrementing solution count, current max matches: {}",
                    max_matches
                );
            }
        } else if grab < passenger {
            println!(
                "    Grab index {} is less than Passenger index {}, incrementing Grab index",
                grab, passenger
            );
            i += 1;
        } else {
            println!(
                "    Passenger index {} is less than Grab index {}, incrementing Passenger index",
                passenger, grab
            );
            j += 1;
        }
    }

    println!("> Efficient search complete.");
    println!(
        "> Max Matches: {}, Solutions: {}\n",
        max_matches, total_solutions
    );
    (max_matches, total_solutions)
}
