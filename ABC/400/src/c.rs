use proconio::input;

fn main() {
    input! {
        n: u128,
    }

    let mut res = 0;
    let mut r = 1;
    while (1_u128 << r) <= n {
        let upper = (n as u128) / (1_u128 << r);

        let t = sqrt(upper);
        res += ((t + 1) / 2) as u128;

        r += 1;
    }

    println!("{}", res);
}
fn sqrt(x: u128) -> u128 {
    let mut left = 0_u128;
    let mut right = 1_u128 << 65;
    while right - left > 1 {
        let mid = (left + right) >> 1;
        if mid <= x / mid {
            left = mid;
        } else {
            right = mid;
        }
    }
    left
}
