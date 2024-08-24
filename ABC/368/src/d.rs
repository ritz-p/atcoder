use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize,usize);n-1],
        v: [usize;k]
    };

    let mut uf = UnionFind::new(n + 1);

    let mut required_set = vec![false; n + 1];
    for &vi in &v {
        required_set[vi] = true;
    }

    for (a, b) in ab {
        uf.union(a, b);
    }
    let root = uf.find(v[0]);
    let result = uf.size(root);
    println!("{}", result);
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..=n).collect(),
            size: vec![1; n + 1],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            if self.size[root_x] < self.size[root_y] {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
            } else {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            }
            true
        } else {
            false
        }
    }

    fn size(&mut self, x: usize) -> usize {
        let root_x = self.find(x);
        self.size[root_x]
    }
}
