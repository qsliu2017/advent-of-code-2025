fn main() {
    let groups = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .filter(|line| !line.is_empty())
        .fold(vec![], |mut groups, line| {
            line.split_ascii_whitespace()
                .enumerate()
                .for_each(|(i, el)| {
                    if groups.len() <= i {
                        groups.push(vec![el.to_owned()]);
                    } else {
                        groups[i].push(el.to_owned());
                    }
                });
            groups
        });

    let ans = groups
        .into_iter()
        .map(|group| match group.last().unwrap().as_str() {
            "+" => group[..group.len() - 1]
                .into_iter()
                .map(|el| el.parse::<usize>().unwrap())
                .sum::<usize>(),
            "*" => group[..group.len() - 1]
                .into_iter()
                .map(|el| el.parse::<usize>().unwrap())
                .product::<usize>(),
            _ => unreachable!(),
        })
        .sum::<usize>();

    println!("{}", ans);
}
