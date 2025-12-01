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
            let pos = match dir {
                'R' => (pos + dist) % 100,
                'L' => (pos - dist).rem_euclid(100),
                _ => unreachable!(),
            };

            (pos, count + if pos == 0 { 1 } else { 0 })
        });

    println!("{}", count);
}
