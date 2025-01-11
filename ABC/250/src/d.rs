use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let limit = 1_000_002;
    let mut prime = vec![true; limit];
    let mut i = 2;
    let mut res = 0;
    let mut ns = vec![0; limit];
    while i * i * i <= n {
        ns[i] = ns[i - 1];
        if prime[i] {
            let mut j = 2;
            while i * j < limit {
                prime[i * j] = false;
                j += 1;
            }
            let x = n / (i * i * i);
            if x > i {
                res += ns[i];
            } else {
                res += ns[x];
            }
            ns[i] += 1;
        }
        i += 1;
    }
    println!("{}", res);
}
