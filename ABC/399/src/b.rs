use proconio::input;
fn main() {
    input! {
        n: usize,
        p: [usize;n]
    };
    let mut v = vec![];

    for i in 0..n {
        v.push((p[i], i));
    }
    v.sort_by(|a, b| b.0.cmp(&a.0));

    let mut current = 1;
    let mut res = vec![];
    res.push((current, v[0].1));
    for i in 1..n {
        if v[i].0 == v[i - 1].0 {
            res.push((current, v[i].1));
        } else {
            current = i + 1;
            res.push((current, v[i].1));
        }
    }
    res.sort_by(|a, b| a.1.cmp(&b.1));

    for e in res {
        println!("{}", e.0);
    }
}
