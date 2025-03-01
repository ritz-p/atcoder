use proconio::input;
fn main() {
    input! {
        n: usize,
        q: usize,
    };
    let mut pigeons = vec![0; n];
    let mut nest = vec![0; n];
    let mut rev = vec![0; n];
    for i in 0..n {
        pigeons[i] = i;
        nest[i] = i;
        rev[i] = i;
    }
    for _i in 0..q {
        input! {
            op: usize,
        };
        match op {
            1 => {
                input! {
                    a: usize,
                    b: usize,
                };
                pigeons[a - 1] = nest[b - 1];
            }
            2 => {
                input! {
                    a: usize,
                    b: usize,
                };
                let an = nest[a - 1];
                let bn = nest[b - 1];
                nest[a - 1] = bn;
                nest[b - 1] = an;
                rev[bn] = a - 1;
                rev[an] = b - 1;
            }
            3 => {
                input! {
                    a: usize
                };
                let pigeon = pigeons[a - 1];
                let pn = rev[pigeon];
                println!("{}", pn + 1);
            }
            _ => {}
        }
    }
}
