use std::collections::BTreeMap;

fn main() {
    let ans = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            let start = start.parse::<usize>().unwrap();
            let end = end.parse::<usize>().unwrap();
            (start, end)
        })
        .fold(BTreeMap::new(), |mut range_tree, (start, end)| {
            // merge overlap range
            let (start, end) = range_tree
                .range(start..=end)
                .fold((start, end), |(start, end), (&one, &other)| {
                    (start.min(one).min(other), end.max(one).max(other))
                });

            // if one range already include us, just skip
            if let Some((&next_end, &next_start)) = range_tree.range(end..).next()
                && next_start <= next_end
                && next_start <= start
                && end <= next_end
            {
                return range_tree;
            }

            range_tree.retain(|&k, _| k < start || end < k);

            range_tree.insert(start, end);
            range_tree.insert(end, start);
            range_tree
        })
        .into_iter()
        .map(|(k, v)| if k <= v { v - k + 1 } else { 0 })
        .sum::<usize>();

    println!("{}", ans);
}
