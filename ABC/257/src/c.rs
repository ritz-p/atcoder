use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        w: [usize;n]
    };
    let mut ws = vec![];
    let mut res = 0;
    for i in 0..n {
        ws.push((s[i], w[i]));
        match s[i] {
            '1' => {
                res += 1;
            }
            _ => {}
        }
    }
    ws.sort_by(|a, b| a.1.cmp(&b.1));

    let mut x = res;
    for i in 0..n {
        if ws[i].0 == '1' {
            x -= 1;
        } else {
            x += 1;
        }
        if i < n - 1 {
            if ws[i].1 != ws[i + 1].1 {
                res = res.max(x);
            }
        } else {
            res = res.max(x);
        }
    }
    println!("{}", res);
}
