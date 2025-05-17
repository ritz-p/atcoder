use proconio::input;
fn main() {
    input! {
        n: usize,
        p: [usize;n]
    };
    if n < 4 {
        println!("0");
        return;
    }

    let mut pm: Vec<(usize, usize)> = vec![];
    for i in 0..n - 1 {
        let dir = if p[i] < p[i + 1] { 1 } else { 0 };
        if let Some(last) = pm.last_mut() {
            if last.0 == dir {
                last.1 += 1;
                continue;
            }
        }
        pm.push((dir, 1));
    }

    let mut res = 0;
    for w in pm.windows(3) {
        if w[0].0 == 1 && w[1].0 == 0 && w[2].0 == 1 {
            res += w[0].1 * w[2].1;
        }
    }

    println!("{}", res);
}
