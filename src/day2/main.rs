use std::io::Read;

fn main() {
    let mut buf = vec![0_u8; 1024];
    let len = std::io::stdin().read(&mut buf).unwrap();
    assert!(len < 1024);
    let bounds = buf[..len]
        .split(|&ch| ch == b',' || ch == b'-')
        .collect::<Vec<_>>();
    assert!(bounds.len() % 2 == 0);
    let ans: usize = bounds
        .chunks(2)
        .flat_map(|chunk| invalid_ids_in_range(&chunk[0], &chunk[1]))
        .sum();
    println!("{}", ans);
}

fn invalid_ids_in_range(from: &[u8], to: &[u8]) -> Vec<usize> {
    assert!(from.len() <= to.len());
    if from.len() < to.len() {
        return (from.len() + 1..to.len())
            .flat_map(|d| invalid_ids_in_range(&minimun_of_digits(d), &maximun_of_digits(d)))
            .chain(invalid_ids_in_range(from, &maximun_of_digits(from.len())))
            .chain(invalid_ids_in_range(&minimun_of_digits(to.len()), to))
            .collect();
    }
    if from.len() % 2 != 0 {
        return vec![];
    }
    let from_hi: usize = String::from_utf8_lossy(&from[..from.len() / 2])
        .parse()
        .unwrap();
    let from_lo: usize = String::from_utf8_lossy(&from[from.len() / 2..])
        .parse()
        .unwrap();
    let to_hi: usize = String::from_utf8_lossy(&to[..to.len() / 2])
        .parse()
        .unwrap();
    let to_lo: usize = String::from_utf8_lossy(&to[to.len() / 2..])
        .parse()
        .unwrap();
    let mut v = vec![];
    if from_hi >= from_lo && (from_hi < to_hi || from_hi <= to_lo)  {
        v.push(from_hi * (10_usize.pow((from.len() / 2) as u32) + 1));
    }
    v.extend((from_hi + 1..to_hi).map(|hi| hi * (10_usize.pow((from.len() / 2) as u32) + 1)));
    if from_hi < to_hi && to_hi <= to_lo {
        v.push(to_hi * (10_usize.pow((from.len() / 2) as u32) + 1));
    }
    v
}

#[inline]
fn minimun_of_digits(d: usize) -> Vec<u8> {
    let mut v = vec![b'0'; d];
    v[0] = b'1';
    v
}

#[inline]
fn maximun_of_digits(d: usize) -> Vec<u8> {
    vec![b'9'; d]
}
