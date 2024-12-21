use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize;n]
    };
    let mut res = 1;
    for i in 0..n {
        let current = h[i];
        for j in 1..n {
            let mut count = 0;
            for k in (i..n).step_by(j) {
                if h[k] == current {
                    count += 1;
                } else {
                    break;
                }
            }
            res = res.max(count);
        }
    }
    println!("{}", res);
}
