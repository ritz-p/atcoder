use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let mut v = vec![0; n];

    for b in 0..m {
        input! {
            k: usize,
            s: [usize;k]
        };

        for i in &s {
            v[*i - 1] |= 1 << b;
        }
    }

    input! {
        p: [usize;m]
    };

    let target: usize = (0..m).map(|i| p[i] << i).sum();

    let mut res = 0;
    for bit in 0..(1 << n) {
        let mut test = 0;
        for i in 0..n {
            if (bit >> i) & 1 == 1 {
                test ^= v[i];
            }
        }
        if test == target {
            res += 1;
        }
    }

    println!("{}", res);
}
