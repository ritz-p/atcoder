use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    };
    let mut pos = vec![Vec::new(); 26];
    for (i, &c) in s.iter().enumerate() {
        let index = (c as u8 - b'A') as usize;
        pos[index].push(i);
    }

    let mut res = 0;

    for p in pos.iter() {
        let m = p.len();
        if m < 2 {
            continue;
        }
        let mut sum = vec![0usize; m];
        sum[0] = p[0];
        for i in 1..m {
            sum[i] = sum[i - 1] + p[i];
        }

        let mut diff = 0;
        for k in 1..m {
            let pi = sum[k - 1];
            let pk = p[k];
            diff += pk * k - pi;
        }

        let pairs = m * (m - 1) / 2;
        res += diff - pairs;
    }

    println!("{}", res);
}
