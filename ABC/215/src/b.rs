use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut k = 0;

    while 2_usize.pow(k) <= n {
        k += 1;
    }

    println!("{}", k - 1);
}
