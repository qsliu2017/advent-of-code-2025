fn main() {
    let ans: usize = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .map(|line| {
            let dp =
                line.bytes()
                    .map(|ch| (ch - b'0') as usize)
                    .fold([None; 12], |mut dp, digit| {
                        for i in (0..11).rev() {
                            if let Some(prev) = dp[i] {
                                dp[i + 1] = Some(dp[i + 1].unwrap_or(0).max(prev * 10 + digit));
                            }
                        }
                        dp[0] = Some(dp[0].unwrap_or(0).max(digit));
                        dp
                    });
            dp[11].unwrap()
        })
        .inspect(|x| {
            dbg!(x);
        })
        .sum();

    println!("{}", ans);
}
