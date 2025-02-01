use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    };
    let mut ps = vec![HashSet::<usize>::new(); n + 1];
    let mut v = vec![0; n + 1];
    for i in 0..n + 1 {
        v[i] = i;
        ps[i].insert(i);
    }
    let mut set = HashSet::new();

    for _i in 0..q {
        input! {
            t: usize
        };
        match t {
            1 => {
                input! {
                    p: usize,
                    h: usize
                };
                ps[v[p]].remove(&p);
                ps[h].insert(p);
                if ps[h].len() >= 2 {
                    set.insert(h);
                }
                if ps[v[p]].len() <= 1 {
                    set.remove(&v[p]);
                }
                v[p] = h;
            }
            2 => {
                println!("{}", set.len());
            }
            _ => {}
        }
    }
}
