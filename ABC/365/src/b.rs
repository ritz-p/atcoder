use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    };
    let mut v = vec![];
    for (index, e) in a.iter().enumerate() {
        v.push((e, index));
    }
    v.sort_by(|a, b| b.0.cmp(&a.0));
    println!("{}", v[1].1 + 1);
}
