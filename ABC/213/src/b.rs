use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    };
    let mut v = vec![];
    for (index, e) in a.iter().enumerate() {
        v.push((e, index))
    }
    v.sort_by(|x, y| x.0.cmp(&y.0));

    println!("{}", v[n - 2].1 + 1);
}
