fn main() {
    let (_, count) = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (dir, dist_str) = l.split_at(1);
            (
                dir.chars().next().unwrap(),
                dist_str.parse::<i32>().unwrap(),
            )
        })
        .fold((50, 0), |(pos, count), (dir, dist)| {
            let count = count
                + match dir {
                    'R' => (pos + dist) / 100,
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
                    _ => unreachable!(),
                };
            let pos = match dir {
                'R' => (pos + dist) % 100,
                'L' => (pos - dist).rem_euclid(100),
                _ => unreachable!(),
            };
            (pos, count)
        });

    println!("{}", count);
}
