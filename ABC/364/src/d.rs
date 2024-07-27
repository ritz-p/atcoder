use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [isize;n],
        bk: [(isize,usize);q]
    };
    for (b, k) in &bk {
        let pos = a.binary_search(&b).unwrap_or_else(|x| x);
        let mut current = 0;
        let mut plus = 1;
        let mut minus = 1;
        while current < *k {
            if pos - minus > 0 && pos + plus < n - 1 {
                if (b - a[pos - minus]).abs() < (b - a[pos + plus]).abs() {
                    minus += 1;
                } else {
                    plus += 1;
                }
                current += 1
            }
        }
        println!("{}", a[pos]);
    }
}
