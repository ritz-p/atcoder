use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };

    println!("{}", gcd(a, b));
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
