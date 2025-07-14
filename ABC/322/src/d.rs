use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        p1: [Chars; 4],
        p2: [Chars; 4],
        p3: [Chars; 4],
    };

    let pv1 = collect(&p1);
    let pv2 = collect(&p2);
    let pv3 = collect(&p3);

    if pv1.len() + pv2.len() + pv3.len() != 16 {
        println!("No");
        return;
    }

    for r1 in 0..4 {
        let pc1 = init(&rotate(&pv1, r1));
        let (h1, w1) = end(&pc1);

        for dx1 in 0..=4 - h1 {
            for dy1 in 0..=4 - w1 {
                let placed1: HashSet<_> = pc1.iter().map(|&(x, y)| (x + dx1, y + dy1)).collect();

                for r2 in 0..4 {
                    let pc2 = init(&rotate(&pv2, r2));
                    let (h2, w2) = end(&pc2);

                    for dx2 in 0..=4 - h2 {
                        for dy2 in 0..=4 - w2 {
                            let placed2: Vec<_> =
                                pc2.iter().map(|&(x, y)| (x + dx2, y + dy2)).collect();
                            if placed2.iter().any(|c| placed1.contains(c)) {
                                continue;
                            }
                            let mut used12 = placed1.clone();
                            used12.extend(placed2.iter().cloned());

                            for r3 in 0..4 {
                                let pc3 = init(&rotate(&pv3, r3));
                                let (h3, w3) = end(&pc3);

                                for dx3 in 0..=4 - h3 {
                                    for dy3 in 0..=4 - w3 {
                                        let placed3: Vec<_> =
                                            pc3.iter().map(|&(x, y)| (x + dx3, y + dy3)).collect();
                                        if placed3.iter().any(|c| used12.contains(c)) {
                                            continue;
                                        }
                                        if used12.len() + placed3.len() == 16 {
                                            println!("Yes");
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("No");
}

fn collect(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut v = Vec::new();
    for i in 0..4 {
        for j in 0..4 {
            if grid[i][j] == '#' {
                v.push((i, j));
            }
        }
    }
    v
}

fn rotate(cells: &[(usize, usize)], k: usize) -> Vec<(usize, usize)> {
    let mut res = cells.to_vec();
    for _ in 0..k {
        res = res.into_iter().map(|(x, y)| (y, 3 - x)).collect();
    }
    res
}
fn init(cells: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let min_x = cells.iter().map(|&(x, _)| x).min().unwrap();
    let min_y = cells.iter().map(|&(_, y)| y).min().unwrap();
    cells.iter().map(|&(x, y)| (x - min_x, y - min_y)).collect()
}

fn end(cells: &[(usize, usize)]) -> (usize, usize) {
    let h = cells.iter().map(|&(x, _)| x).max().unwrap() + 1;
    let w = cells.iter().map(|&(_, y)| y).max().unwrap() + 1;
    (h, w)
}
