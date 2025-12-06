fn main() {
    let rows = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let (rows, ops) = rows.split_at(rows.len() - 1);

    let max_len = rows.iter().map(|row| row.len()).max().unwrap();

    let cols = (0..max_len)
        .map(|i| {
            rows.iter()
                .filter_map(|row| (i < row.len()).then(|| row.as_bytes()[i]))
                .filter(|ch| ch.is_ascii_digit())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let ans = cols
        .split(|col| col.is_empty())
        .map(|elems| {
            elems
                .iter()
                .map(|col| str::from_utf8(col).unwrap().parse::<usize>().unwrap())
        })
        .zip(ops[0].split_ascii_whitespace())
        .map(|(elems, op)| match op {
            "+" => elems.sum::<usize>(),
            "*" => elems.product::<usize>(),
            _ => unreachable!(),
        })
        .sum::<usize>();

    println!("{}", ans);
}
