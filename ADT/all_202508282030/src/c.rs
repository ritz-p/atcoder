use proconio::{input, marker::Chars};
fn main() {
    input! {
        _n: usize,
        s: Chars
    };
    let mut res = (0, 0);
    let directions = vec![(1, 0), (0, -1), (-1, 0), (0, 1)];
    let mut f = 0;
    for c in s {
        match c {
            'S' => {
                res = (res.0 + directions[f].0, res.1 + directions[f].1);
            }
            'R' => {
                f += 1;
                f %= 4;
            }
            _ => {}
        }
    }

    println!("{} {}", res.0, res.1);
}
