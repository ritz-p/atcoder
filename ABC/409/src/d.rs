use proconio::{input, marker::Chars};
fn main() {
    input! {
        t: usize,
    };
    let mut case = vec![];
    for _i in 0..t {
        input! {
            n: usize,
            s: Chars
        };
        case.push((n, s));
    }
    for (n, s) in case {
        let mut start = 0;
        for i in 0..n - 1 {
            if s[i] > s[i + 1] {
                start = i;
                break;
            }
        }
        let mut end = n;
        for i in start + 1..n {
            if s[start] < s[i] {
                end = i;
                break;
            }
        }

        let mut res = vec![];
        res.extend_from_slice(&s[0..start]);
        res.extend_from_slice(&s[start + 1..end]);
        res.push(s[start]);
        res.extend_from_slice(&s[end..n]);
        println!("{}", res.iter().collect::<String>());
    }
}
