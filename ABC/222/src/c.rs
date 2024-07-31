use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars;2*n]
    };
    let mut v = vec![];
    for i in 0..2 * n {
        v.push((i, 0));
    }
    for i in 0..m {
        for j in 0..n {
            if a[v[2 * j].0][i] == a[v[2 * j + 1].0][i] {
                continue;
            }
            if a[v[2 * j].0][i] == 'G' && a[v[2 * j + 1].0][i] == 'C'
                || a[v[2 * j].0][i] == 'C' && a[v[2 * j + 1].0][i] == 'P'
                || a[v[2 * j].0][i] == 'P' && a[v[2 * j + 1].0][i] == 'G'
            {
                v[2 * j].1 += 1;
            } else {
                v[2 * j + 1].1 += 1;
            }
        }
        v.sort_by(|a, b| {
            let ord = b.1.cmp(&a.1);
            if ord == std::cmp::Ordering::Equal {
                a.0.cmp(&b.0)
            } else {
                ord
            }
        });
    }
    for i in 0..2 * n {
        println!("{}", v[i].0 + 1);
    }
}
