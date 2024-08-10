use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        n: usize,
        s: [Chars;n]
    };
    let mut m = 0;
    for i in &s {
        if i.len() > m {
            m = i.len();
        }
    }
    let mut v = vec![vec![]; m];
    for i in &s {
        for (index, c) in i.iter().enumerate() {
            v[index].push(c);
        }
        for j in i.len()..m {
            v[j].push(&'*');
        }
    }
    for l in &mut v {
        let mut found_non_asterisk = false;
        l.retain(|c| {
            if **c != '*' && !found_non_asterisk {
                found_non_asterisk = true;
            }
            found_non_asterisk || **c != '*'
        });
    }

    for l in v {
        println!("{}", l.iter().rev().join(""));
    }
}
