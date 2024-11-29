use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        q: usize,
        tk: [(usize,usize);q]
    };
    let v = s
        .iter()
        .map(|x| (*x as usize) - ('A' as usize))
        .collect::<Vec<usize>>();
    for (t, k) in tk {
        println!("{}", ((solve(t, k - 1, &v) as u8) + ('A' as u8)) as char);
    }
}

fn solve(t: usize, k: usize, s: &Vec<usize>) -> usize {
    if t == 0 {
        return s[k];
    }
    if k == 0 {
        return (s[0] + t) % 3;
    }
    return (solve(t - 1, k / 2, s) + k % 2 + 1) % 3;
}
