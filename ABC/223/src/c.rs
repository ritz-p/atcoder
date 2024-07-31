use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize,usize);n]
    };
    let mut total = 0.0;
    for (a, b) in &ab {
        total += *a as f64 / *b as f64;
    }
    let mut res = 0.0;
    let mut time = total / 2.0;
    for (a, b) in ab {
        if time >= a as f64 / b as f64 {
            time -= a as f64 / b as f64;
            res += a as f64;
        } else {
            res += b as f64 * time;
            break;
        }
    }
    println!("{:.15}", res);
}
