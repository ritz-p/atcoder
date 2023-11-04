use proconio::*;
use proconio::marker::*;
use petgraph::unionfind::UnionFind;
fn main() {
    input!(h: usize, w: usize, q: usize);
    let pos = |x: usize, y: usize| x * w + y;
    let mut red = vec![vec![false; w]; h];
    let mut dsu = UnionFind::new(h * w);
    for _ in 0..q {
        input!(op: u8);
        if op == 1 {
            input!(x: Usize1, y: Usize1);
            red[x][y] = true;
            for &(dx, dy) in [(1, 0), (!0, 0), (0, 1), (0, !0)].iter() {
                let p = x.wrapping_add(dx);
                let q = y.wrapping_add(dy);
                if p < h && q < w && red[p][q] {
                    dsu.union(pos(x, y), pos(p, q));
                }
            }
        } else {
            input!(x: Usize1, y: Usize1, z: Usize1, w: Usize1);
            let ans = if red[x][y] && dsu.equiv(pos(x, y), pos(z, w)) {
                "Yes"
            } else {
                "No"
            };
            println!("{}", ans);
        }
    }
}
