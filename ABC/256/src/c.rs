use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
    };
    let mut res = 0;
    for i in 1..=30 {
        for j in 1..=30 {
            for k in 1..=30 {
                for l in 1..=30 {
                    let m = a.saturating_sub(i + j);
                    if m == 0 {
                        continue;
                    }
                    let n = b.saturating_sub(k + l);
                    if n == 0 {
                        continue;
                    }
                    let o = d.saturating_sub(i + k);
                    if o == 0 {
                        continue;
                    }
                    let p = e.saturating_sub(j + l);
                    if p == 0 {
                        continue;
                    }
                    let q = f.saturating_sub(m + n);
                    if q == 0 {
                        continue;
                    }
                    if q + o + p == c {
                        res += 1;
                    }
                }
            }
        }
    }
    println!("{}", res);
}
