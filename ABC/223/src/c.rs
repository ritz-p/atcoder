use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(f64,f64);n]
    };

    let mut total = 0.0;
    let mut res = 0.0;

    for (a, b) in &ab {
        total += a / b;
    }
    let mut time = total / 2.0;
    for (a, b) in &ab {
        if time >= a / b {
            time -= a / b;
            res += a;
        } else {
            res += time * b;
            break;
        }
    }

    println!("{:.15}", res);
}
