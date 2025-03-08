use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        mut b: [isize;n],
        mut w: [isize;m]
    };
    b.sort();
    b.reverse();
    w.sort();
    w.reverse();
    let mut res = 0;
    let mut bc = 0;
    for i in 0..n {
        if b[i] >= 0 {
            res += b[i];
            bc += 1;
        } else {
            break;
        }
    }
    let mut wc = 0;
    for i in 0..m {
        if w[i] >= 0 {
            if wc < bc {
                res += w[i];
            }
            wc += 1;
        } else {
            break;
        }
    }
    if wc > bc {
        for i in bc..wc {
            if n > i {
                if w[i] + b[i] > 0 {
                    res += w[i] + b[i];
                }
            } else {
                break;
            }
        }
    }
    println!("{}", res);
}
