use std::mem::swap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut abc: [(usize,usize,usize);m]
    };
    let mut res = 0;
    let mut uf = UnionFind::new(n + 1);
    abc.sort_by(|a, b| (&b.2).cmp(&a.2));
    for (a, b, c) in abc {
        if uf.is_same(a, b) {
            continue;
        }
        uf.unite(a, b);
        res += c;
    }

    println!("{}", res);
}

pub struct UnionFind {
    pub parent: Vec<usize>,
    pub size: Vec<usize>,
}
impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: vec![usize::MAX; n],
            size: vec![1; n],
        }
    }
    pub fn root(&mut self, x: usize) -> usize {
        if self.parent[x] == usize::MAX {
            return x;
        }
        self.root(self.parent[x])
    }

    pub fn unite(&mut self, u: usize, v: usize) -> bool {
        let mut root_u = self.root(u);
        let mut root_v = self.root(v);

        if root_u == root_v {
            return false;
        }

        if self.size[root_u] < self.size[root_v] {
            swap(&mut root_u, &mut root_v);
        }

        self.parent[root_v] = root_u;
        self.size[root_u] += self.size[root_v];

        true
    }

    pub fn is_same(&mut self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }

    pub fn size(&mut self, x: usize) -> usize {
        let r = self.root(x);
        self.size[r]
    }
}
