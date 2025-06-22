use proconio::input;
fn main() {
    input! {
        n: usize,
        d: [usize;n-1]
    };

    for i in 0..n - 2 {
        let mut current = 0;
        for j in i..n - 1 {
            current += d[j];
            print!("{} ", current);
        }
        println!();
    }
    println!("{}", d[n - 2]);
}
