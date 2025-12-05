use std::{collections::HashSet, iter};

fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .map(|line| {
            let line = iter::once(false)
                .chain(line.bytes().map(|ch| ch == b'@'))
                .chain(iter::once(false))
                .collect::<Vec<_>>();

            line.windows(3)
                .map(|w| (w[1], w.into_iter().filter(|&&x| x).count() as i8))
                .collect::<Vec<_>>()
        });

    const MAX_WIDTH: usize = 160;
    let lines = iter::once(vec![(false, 0); MAX_WIDTH])
        .chain(lines)
        .chain(iter::once(vec![(false, 0); MAX_WIDTH]))
        .collect::<Vec<_>>();

    let mut map = lines
        .windows(3)
        .map(|w| {
            iter::zip(&w[0], iter::zip(&w[1], &w[2]))
                .map(|((_, a), (&(is_at, b), (_, c)))| {
                    if is_at {
                        a + b + c - 1 /* ME! */
                    } else {
                        -1
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let n_row = map.len();
    let n_col = map[0].len();

    let mut to_remove = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, &cnt)| (cnt != -1 && cnt < 4).then_some((i, j)))
        })
        .collect::<Vec<_>>();

    let mut ans = 0;

    let dx_dy = [-1, 0, 1]
        .into_iter()
        .flat_map(|dx| {
            [-1, 0, 1]
                .into_iter()
                .filter_map(move |dy| (dx != 0 || dy != 0).then_some((dx, dy)))
        })
        .collect::<Vec<_>>();

    let nearby = |(i, j)| {
        dx_dy.iter().filter_map(move |(dx, dy)| {
            let i = i as isize;
            let j = j as isize;
            (0 <= i + dx && i + dx < n_row as isize && 0 <= j + dy && j + dy < n_col as isize)
                .then_some(((i + dx) as usize, (j + dy) as usize))
        })
    };

    while !to_remove.is_empty() {
        ans += to_remove.len();
        to_remove.iter().for_each(|&(i, j)| {
            map[i][j] = -1;
        });
        to_remove = to_remove
            .into_iter()
            .flat_map(nearby)
            .filter(|&(i, j)| {
                map[i][j] != -1 && {
                    map[i][j] -= 1;
                    map[i][j] < 4
                }
            })
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
    }

    println!("{}", ans);
}
