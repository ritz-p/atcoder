use proconio::input;
fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let mut g = gcd(a, b);
    let mut res = 1;
    for p in 2..=1_000_000 {
        if g % p == 0 {
            res += 1;
            while g % p == 0 {
                g /= p;
            }
        }
    }
    if g > 1 {
        res += 1;
    }

    println!("{}", res);
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
