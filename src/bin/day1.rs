use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut pos: i32 = 50; // Current dial position (0-99)
    let mut count: u32 = 0; // Counter for times the dial lands on 0

    // Process each rotation line
    for line in input.lines().filter(|l| !l.is_empty()) {
        // Split line into direction (first char) and distance (rest of string)
        let (dir, dist_str) = line.split_at(1);
        let dist: i32 = dist_str.parse().unwrap();

        // Calculate the new position
        pos = match dir.chars().next().unwrap() {
            'R' => (pos + dist) % 100,
            // 'L' uses rem_euclid(100) to ensure the result is positive (e.g., -5 % 100 is 95)
            'L' => (pos - dist).rem_euclid(100),
            _ => pos, // Should not happen with valid input
        };

        // Check if the actual password condition is met
        if pos == 0 {
            count += 1;
        }
    }

    println!("{}", count);
}
