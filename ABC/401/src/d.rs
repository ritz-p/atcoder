use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut k: usize,
        mut s: Chars
    };

    for i in 0..n {
        if s[i] == 'o' {
            k -= 1;
            if i > 0 && s[i - 1] == '?' {
                s[i - 1] = '.';
            }
            if i < n - 1 && s[i + 1] == '?' {
                s[i + 1] = '.';
            }
        }
    }
    let mut v = vec![];
    let mut l = 0;
    let mut m = 0;

    for i in 0..n {
        if s[i] == '?' {
            l += 1;
        } else {
            v.push((i - l, l));
            m += (l + 1) / 2;
            l = 0;
        }
    }
    if l > 0 {
        m += (l + 1) / 2;
        v.push((n - l, l));
    }

    if k == m {
        for (i, l) in v {
            if l % 2 == 0 {
                continue;
            }
            for j in 0..l {
                if j % 2 == 0 {
                    s[i + j] = 'o';
                } else {
                    s[i + j] = '.';
                }
            }
        }
    } else if k == 0 {
        s = s
            .iter()
            .map(|c| if *c == '?' { '.' } else { *c })
            .collect::<Vec<char>>();
    }

    println!("{}", s.iter().collect::<String>());
}
