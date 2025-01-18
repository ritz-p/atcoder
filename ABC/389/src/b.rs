use proconio::input;

fn main() {
    input! {
        x: usize,
    };
    let mut current = 1;
    let mut res = 1;
    while current != x {
        res += 1;
        current *= res;
    }
    println!("{}", res);
}
