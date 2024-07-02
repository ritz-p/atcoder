use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    };
    let mut a = 1;
    let mut b = 1;
    let m = 1000000007;
    for i in 1..=h + w - 2 {
        a = (a * i) % m;
    }

    for i in 1..=h - 1 {
        b = (b * i) % m;
    }
    for i in 1..=w - 1 {
        b = (b * i) % m;
    }
    println!("{}", div(a, b, m));
}

fn div(a: usize, b: usize, m: usize) -> usize {
    (a * pow(b, m - 2, m)) % m
}

fn pow(a: usize, b: usize, m: usize) -> usize {
    let mut p = a;
    let mut res = 1;
    for i in 0..30 {
        let w = 1 << i;
        if (b / w) % 2 == 1 {
            res = (res * p) % m;
        }
        p = (p * p) % m
    }
    res
}
