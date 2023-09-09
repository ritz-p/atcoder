use itertools::Itertools;
use proconio::input;

fn main() {
    let rows = vec![
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    let mut all = 0;
    let mut sum = 0;
    for order in (0..9).permutations(9) {
        all += 1;

        for &[a1, a2, a3] in rows.iter() {
            if c[a1] == c[a2] && c[a1] != c[a3] && order[a1] < order[a3] && order[a2] < order[a3] {
                sum += 1;
                break;
            }

            if c[a1] == c[a3] && c[a1] != c[a2] && order[a1] < order[a2] && order[a3] < order[a2] {
                sum += 1;
                break;
            }

            if c[a2] == c[a3] && c[a1] != c[a2] && order[a2] < order[a1] && order[a3] < order[a1] {
                sum += 1;
                break;
            }
        }
    }

    println!("{}", (all - sum) as f64 / all as f64);
}
