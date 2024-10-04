use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize;n],
        x: [usize;q]
    };
    a.sort();
    for query in x {
        let point = a.partition_point(|v| *v < query);
        println!("{}", n - point);
    }
}
