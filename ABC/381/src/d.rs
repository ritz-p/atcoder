use proconio::input;

fn main() {
    input! {
        n: usize,
        arr: [usize;n]
    };
    let mut res = 0;
    for si in 0..2 {
        let mut count: Vec<usize> = vec![0; n + 1];
        let mut r = si;
        for l in (si..n).step_by(2) {
            while r < n - 1 {
                if arr[r] != arr[r + 1] {
                    break;
                }
                if count[arr[r]] != 0 {
                    break;
                }
                count[arr[r]] += 1;
                r += 2;
            }
            res = res.max(r - l);
            if l == r {
                r += 2;
            } else {
                count[arr[l]] -= 1;
            }
        }
    }

    println!("{}", res);
}
