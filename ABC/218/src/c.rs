use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars;n],
        t: [Chars;n],
    };
    let mut tv = vec![];
    let mut sv = vec![];
    for i in 0..n {
        for j in 0..n {
            if t[i][j] == '#' {
                tv.push((i as isize, j as isize));
            }
            if s[i][j] == '#' {
                sv.push((i as isize, j as isize));
            }
        }
    }
    if sv.len() != tv.len() {
        println!("No");
        return;
    }
    let l = sv.len();
    for _i in 0..4 {
        let diff = (sv[0].0 - tv[0].0, sv[0].1 - tv[0].1);
        let mut f = true;
        for j in 0..l {
            if (sv[j].0 - tv[j].0, sv[j].1 - tv[j].1) != diff {
                f = false;
                break;
            }
        }
        if f {
            println!("Yes");
            return;
        } else {
            sv = rotate(sv, n);
            sv.sort();
        }
    }
    println!("No");
}

fn rotate(sv: Vec<(isize, isize)>, n: usize) -> Vec<(isize, isize)> {
    let mut res = vec![];
    for i in &sv {
        res.push((n as isize - 1 - i.1, i.0));
    }
    res
}
