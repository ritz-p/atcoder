use std::usize;

use proconio::input;
fn main() {
    input! {
        n: usize,
        rc: [(isize,isize);n]
    };
    let mut rmin = isize::MAX;
    let mut rmax = isize::MIN;
    let mut cmin = isize::MAX;
    let mut cmax = isize::MIN;

    for (r, c) in rc {
        if r < rmin {
            rmin = r;
        }
        if r > rmax {
            rmax = r;
        }
        if c < cmin {
            cmin = c;
        }
        if c > cmax {
            cmax = c;
        }
    }

    let rd = rmax - rmin;
    let cd = cmax - cmin;
    let res = ((rd.max(cd)) + 1) / 2;

    println!("{}", res);
}
