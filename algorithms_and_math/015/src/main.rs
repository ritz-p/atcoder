use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
    };

    let mut is_zero = false;
    while !is_zero {
        if a <= b {
            b %= a;
        } else {
            a %= b;
        }

        if a == 0 || b == 0 {
            is_zero = true
        }
    }

    if a <= b {
        println!("{}", b)
    } else {
        println!("{}", a)
    }
}
