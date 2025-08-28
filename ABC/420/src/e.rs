use proconio::input;
fn main() {
    input! {
        n: usize,
        q: usize,
    };
    let mut uf = UnionFind::new(n + 1);

    let mut s = vec![0; n + 1];
    let mut c = vec![0; n + 1];

    for i in 0..q {
        input! {
            t: usize,
        };
        match t {
            1 => {
                input! { mut u: usize, mut v: usize }
                u = uf.leader(u);
                v = uf.leader(v);
                if u != v {
                    let sum = s[u] + s[v];
                    uf.merge(u, v);
                    let w = uf.leader(u);
                    s[w] = sum;
                }
            }
            2 => {
                input! { u: usize }
                let r = uf.leader(u);
                s[r] -= c[u] as usize;
                c[u] ^= 1;
                s[r] += c[u] as usize;
            }
            3 => {
                input! { u: usize }
                let r = uf.leader(u);
                if s[r] > 0 {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
            _ => {}
        }
    }
}
pub struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n);
        for i in 0..n {
            parent.push(i);
        }
        Self {
            par: parent,
            siz: vec![1; n],
        }
    }
    pub fn leader(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            let r = self.leader(self.par[x]);
            self.par[x] = r;
            r
        }
    }
    fn merge(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.leader(x);
        let mut y = self.leader(y);
        if x == y {
            return false;
        }
        if self.siz[x] < self.siz[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.par[y] = x;
        self.siz[x] += self.siz[y];
        true
    }
    pub fn size(&mut self, x: usize) -> usize {
        let r = self.leader(x);
        return self.siz[r as usize];
    }
}
