use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: usize,
    };
    let res = (a + k - 1) % n;
    if res == 0 {
        println!("{}", n);
    } else {
        println!("{}", res);
    }
}
