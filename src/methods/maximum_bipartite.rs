use std::collections::HashSet;

pub fn maximum_bipartite_matching(arr: &Vec<char>, k: usize) -> usize {
    println!("\n=== Maximum Bipartite Matching ===");
    println!("> Starting bipartite matching...");

    let n = arr.len();
    let mut matches = 0;
    let mut grab_cars = Vec::new();
    let mut passengers = Vec::new();

    // Find all 'G' and 'P' positions
    for i in 0..n {
        if arr[i] == 'G' {
            grab_cars.push(i);
        } else if arr[i] == 'P' {
            passengers.push(i);
        }
    }

    let mut adj_list: Vec<HashSet<usize>> = vec![HashSet::new(); grab_cars.len()];
    let mut matched_passengers = vec![None; passengers.len()];

    // Create adjacency list
    for (i, &g_pos) in grab_cars.iter().enumerate() {
        for (j, &p_pos) in passengers.iter().enumerate() {
            if (g_pos as isize - p_pos as isize).abs() as usize <= k {
                adj_list[i].insert(j);
            }
        }
    }

    // DFS to find an augmenting path
    fn dfs(
        u: usize,
        adj_list: &Vec<HashSet<usize>>,
        matched_passengers: &mut Vec<Option<usize>>,
        visited: &mut Vec<bool>,
    ) -> bool {
        if visited[u] {
            return false;
        }
        visited[u] = true;
        for &v in &adj_list[u] {
            if matched_passengers[v] == None
                || dfs(
                    matched_passengers[v].unwrap(),
                    adj_list,
                    matched_passengers,
                    visited,
                )
            {
                matched_passengers[v] = Some(u);
                return true;
            }
        }
        false
    }

    // Find maximum matching
    for u in 0..grab_cars.len() {
        let mut visited = vec![false; grab_cars.len()];
        if dfs(u, &adj_list, &mut matched_passengers, &mut visited) {
            matches += 1;
        }
    }

    println!("> Maximum bipartite matching complete.");
    println!("> Total Matches: {}\n", matches);
    matches
}
