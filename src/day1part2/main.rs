use std::io::Read;

fn main() {
    // Read all input from stdin into a string
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut pos: i32 = 50; // Current dial position (0-99)
    let mut count: u32 = 0; // Counter for total times the dial clicks onto 0

    // Process each rotation line
    for line in input.lines().filter(|l| !l.is_empty()) {
        // Split line into direction (first char) and distance (rest of string)
        let (dir, dist_str) = line.split_at(1);
        let dist: i32 = dist_str.parse().unwrap();

        let zero_hits: i32 = match dir.chars().next().unwrap() {
            'R' => {
                // R: Number of times 0 is clicked is floor((pos + dist) / 100).
                // This accounts for all boundary crossings (multiples of 100).
                (pos + dist) / 100
            }
            'L' => {
                if pos == 0 {
                    // L from 0: Hits 0 only on full 100 cycles.
                    // e.g., L100 hits 0 once, L99 hits 0 zero times.
                    dist / 100
                } else if dist >= pos {
                    // L from pos > 0: Hits 0 on the first 'pos' clicks (including landing on 0),
                    // plus additional full cycles after that.
                    // Total hits = 1 (for the first crossing) + floor((dist - pos) / 100)
                    1 + (dist - pos) / 100
                } else {
                    // L from pos > 0, dist < pos: Never hits 0.
                    0
                }
            }
            _ => 0, // Should not happen with valid input
        };

        // Add the zero clicks that occurred during this rotation
        count += zero_hits as u32;

        // Calculate the new position for the next iteration
        pos = match dir.chars().next().unwrap() {
            'R' => (pos + dist) % 100,
            // 'L' uses rem_euclid(100) to ensure the result is positive (e.g., -5 % 100 is 95)
            'L' => (pos - dist).rem_euclid(100),
            _ => pos, // Should not happen with valid input
        };
    }

    println!("{}", count);
}
