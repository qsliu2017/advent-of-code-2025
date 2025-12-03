use std::cmp::max;

fn main() {
    let ans: usize = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.bytes()
                .fold((b'0', 0), |(max_ch, max_duo), ch| {
                    (
                        max(max_ch, ch),
                        max(max_duo, (max_ch - b'0') * 10 + ch - b'0'),
                    )
                })
                .1 as usize
        })
        .sum();

    println!("{}", ans);
}
