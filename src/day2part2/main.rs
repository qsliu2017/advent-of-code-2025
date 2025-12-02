use std::io::Read;

const MAX_DIGITS: usize = 12;

fn main() {
    let mut buf = vec![0_u8; 1024];
    let len = std::io::stdin().read(&mut buf).unwrap();
    assert!(len < 1024);
    let bounds = buf[..len]
        .split(|&ch| ch == b',' || ch == b'-')
        .map(|digits| String::from_utf8_lossy(digits).parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    assert!(bounds.len() % 2 == 0);

    let repeat_factors_of_digits = (1..=MAX_DIGITS)
        .map(|d| {
            (1..=d / 2)
                .filter(move |&rep_d| d % rep_d == 0)
                .map(move |rep_d| {
                    let base = 10_usize.pow(rep_d as _);
                    (1..d / rep_d).fold(1, |factor, _| factor * base + 1)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let ans: usize = bounds
        .chunks(2)
        .flat_map(|chunk| {
            let (from, to) = (chunk[0], chunk[1]);
            (from..=to).filter(|&n| {
                let digits = (1..=MAX_DIGITS)
                    .find(|&d| n < 10_usize.pow(d as _))
                    .unwrap();
                repeat_factors_of_digits[digits - 1]
                    .iter()
                    .any(|f| n % f == 0)
            })
        })
        .sum();
    println!("{}", ans);
}
