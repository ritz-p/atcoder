use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize;n],
        q: usize,
        x: [usize;q]
    };
    a.sort();

    for e in x {
        println!("{}", lower_bound(&a, e));
    }
}

fn lower_bound(a: &[usize], x: usize) -> usize {
    a.partition_point(|&v| v < x)
}
