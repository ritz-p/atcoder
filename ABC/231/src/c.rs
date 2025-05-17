use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize;n],
        x: [usize;q]
    };
    a.sort();
    for e in x {
        let p = a.partition_point(|v| *v < e);
        println!("{}", n - p);
    }
}
