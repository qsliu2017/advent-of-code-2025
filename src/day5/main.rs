use std::collections::BTreeMap;

fn main() {
    let mut input = std::io::stdin().lines().map(|l| l.unwrap());

    let mut range_tree = BTreeMap::new();
    while let Some(line) = input.next()
        && !line.is_empty()
    {
        let (start, end) = line.split_once('-').unwrap();
        let start = start.parse::<usize>().unwrap();
        let end = end.parse::<usize>().unwrap();
        range_tree
            .entry(start)
            .and_modify(|e1: &mut usize| {
                *e1 = end.max(*e1);
            })
            .or_insert(end);
    }

    let ans = input
        .map(|line| line.parse::<usize>().unwrap())
        .filter(|&id| range_tree.range(..=id).any(|(_, &end)| id <= end))
        .count();

    println!("{}", ans);
}
