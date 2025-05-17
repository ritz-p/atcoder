use std::collections::HashSet;

use proconio::input;
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        xy: [(usize,usize);n],
        q: usize,
        query: [(usize,usize);q]
    };
    let mut xg: Vec<HashSet<usize>> = vec![HashSet::new(); h];
    let mut yg: Vec<HashSet<usize>> = vec![HashSet::new(); w];

    for (x, y) in xy {
        xg[x - 1].insert(y - 1);
        yg[y - 1].insert(x - 1);
    }

    for (q, i) in query {
        match q {
            1 => {
                println!("{}", xg[i - 1].len());
                for v in &xg[i - 1] {
                    yg[*v].remove(&(i - 1));
                }
                xg[i - 1].clear();
            }
            2 => {
                println!("{}", yg[i - 1].len());
                for v in &yg[i - 1] {
                    xg[*v].remove(&(i - 1));
                }
                yg[i - 1].clear();
            }
            _ => {}
        }
    }
}
