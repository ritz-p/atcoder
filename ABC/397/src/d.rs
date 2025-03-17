use proconio::input;
fn main() {
    input! {
        n: i128
    };

    for d in 1.. {
        if d * d * d > n {
            break;
        }
        if n % d != 0 {
            continue;
        }
        let m = n / d;
        let k = f(3, 3 * d, d * d - m);
        if k > 0 {
            println!("{} {}", k + d, k);
            return;
        }
    }
    println!("-1");
}

fn f(a: i128, b: i128, c: i128) -> i128 {
    let mut low = 0;
    let mut high = 600000001;

    while high - low > 1 {
        let mid = (high + low) / 2;
        if a * mid * mid + b * mid + c <= 0 {
            low = mid;
        } else {
            high = mid;
        }
    }
    if a * low * low + b * low + c == 0 {
        return low;
    }
    -1
}
