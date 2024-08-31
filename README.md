# Matching Algorithm Project

## Overview

This project offers a suite of algorithms designed to solve the challenge of optimally matching Grab cars ('G') with Passengers ('P') within a specified maximum distance. Our implementation includes three distinct approaches:

1. **Brute Force Search**: Examines all possible combinations to determine the maximum number of matches.
2. **Greedy Nearest Passenger Search**: Efficiently pairs each Grab car with the nearest available Passenger.
3. **Efficient Search**: Utilizes sorted lists and a two-pointer technique for optimal matching.

## Installation

To get started with this project, ensure you have Rust installed on your system. Then, follow these steps:

```bash
git clone https://github.com/yourusername/matching-algorithm.git
cd matching-algorithm
cargo build
```

## Usage

Incorporate the following functions into your Rust code to utilize the different algorithms:

```rust
fn main() {
    let arr = vec!['G', 'P', 'P', 'G', 'P', 'P', 'G', 'P', 'P', 'G', 'P', 'P', 'G', 'P', 'P', 'G', 'P', 'P', 'G', 'P', 'P', 'G', 'P', 'P', 'G', 'P', 'P', 'G', 'P', 'P', 'G', 'P', 'P', 'G', 'P', 'P', 'G', 'P', 'P', 'G', 'P', 'P', 'G'];
    let k = 2;

    let (max_matches, solutions) = brute_force(&arr, k);
    println!("Brute Force - Max Matches: {}, Solutions: {}", max_matches, solutions);

    let max_matches_greedy = nearest_passenger_greedy(&arr, k);
    println!("Greedy Nearest Passenger - Total Matches: {}", max_matches_greedy);

    let (max_matches_efficient, solutions_efficient) = efficient_search(&arr, k);
    println!("Efficient Search - Max Matches: {}, Solutions: {}", max_matches_efficient, solutions_efficient);
}
```

## Algorithm Details

### 1. Brute Force Search

This comprehensive approach examines every possible combination of Grab cars and Passengers to identify the maximum number of matches within the specified distance `k`.

```rust
fn brute_force(arr: &Vec<char>, k: usize) -> (usize, usize) { ... }
```

**Sample Output:**
```
Brute Force - Max Matches: 4, Solutions: 2
```

### 2. Greedy Nearest Passenger Search

This algorithm iterates through each Grab car, pairing it with the nearest available Passenger within distance `k`. It ensures no Passenger is matched more than once.

```rust
fn nearest_passenger_greedy(arr: &Vec<char>, k: usize) -> usize { ... }
```

**Sample Output:**
```
Greedy Nearest Passenger - Total Matches: 10
```

### 3. Efficient Search

This optimized algorithm first sorts the indices of Grab cars and Passengers, then employs a two-pointer technique to efficiently determine the maximum number of matches within distance `k`.

```rust
fn efficient_search(arr: &Vec<char>, k: usize) -> (usize, usize) { ... }
```

**Sample Output:**
```
Efficient Search - Max Matches: 4, Solutions: 2
```

## Contributing

We welcome contributions! To contribute:

1. Fork the repository
2. Create a new branch for your feature or bug fix
3. Implement your changes and commit them
4. Push to your fork and submit a pull request

## License

This project is licensed under the MIT License. For more details, see the [LICENSE](LICENSE) file.

> **Note:** Adjust the `k` value according to your specific requirements.

> **Warning:** To ensure reliability, test the algorithms with diverse datasets to validate both correctness and performance.