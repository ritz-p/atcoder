use proconio::input;

fn main() {
    input! {
        s: usize,
        t: usize,
    };

    let mut res = 0;

    for a in 0..=100 {
        for b in 0..=100 {
            for c in 0..=100 {
                if a + b + c <= s && a * b * c <= t {
                    res += 1;
                }
            }
        }
    }

    println!("{}", res);
}
