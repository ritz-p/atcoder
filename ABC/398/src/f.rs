use proconio::{input, marker::Chars};
fn main() {
    input! {
        s: Chars,
    };
    let mut cs = vec!['$'];
    for c in &s {
        cs.push(*c);
        cs.push('$');
    }

    let res = manachar(cs);
    let mut k = 0;
    for i in s.len().. {
        if res[i] >= s.len() - k {
            break;
        }
        k += 1;
    }

    print!("{}", s.iter().collect::<String>());
    for i in 0..k {
        print!("{}", s[k - 1 - i]);
    }
    println!();
}

fn manachar(s: Vec<char>) -> Vec<usize> {
    let mut i = 0;
    let mut j = 0;
    let mut res = vec![0; s.len()];
    while i < s.len() {
        while i >= j && i + j < s.len() && s[i - j] == s[i + j] {
            j += 1;
        }
        res[i] = j;
        let mut k = 1;
        while i >= k && k + res[i - k] < j {
            res[i + k] = res[i - k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    res
}
