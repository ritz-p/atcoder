use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        mut x: [usize;n]
    };
    if n == m {
        println!("0");
        return;
    }
    x.sort();

    let mut diff = vec![];
    for i in 0..n - 1 {
        diff.push((x[i + 1] - x[i], i));
    }
    diff.sort_by(|a, b| b.0.cmp(&a.0));

    let mut indexes: Vec<(usize, usize)> = diff.iter().take(m - 1).map(|&v| v).collect();
    indexes.sort_by(|a, b| a.1.cmp(&b.1));

    let mut res = 0;
    let mut pre = 0;

    for (_, index) in &indexes {
        let d = x[*index] - x[pre];
        res += d;
        pre = index + 1;
    }
    res += x[n - 1] - x[pre];

    println!("{}", res);
}
