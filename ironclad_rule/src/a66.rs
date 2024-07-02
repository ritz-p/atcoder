use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [(usize,usize,usize);q]
    };
    let mut parent = vec![-1; n];
    let mut siz = vec![1; n];

    for (query, u, v) in queries {
        if query == 1 {
            unite(u - 1, v - 1, &mut parent, &mut siz);
        } else {
            if is_same(u - 1, v - 1, &parent) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}

fn root(x: usize, parent: &Vec<isize>) -> usize {
    if parent[x] == -1 {
        return x;
    }

    root(parent[x] as usize, parent)
}

fn unite(u: usize, v: usize, parent: &mut Vec<isize>, siz: &mut Vec<usize>) {
    let root_u = root(u, parent);
    let root_v = root(v, parent);

    if root_u == root_v {
        return;
    }

    if siz[root_u] < siz[root_v] {
        parent[root_u] = root_v as isize;
        siz[root_v] += siz[root_u];
    } else {
        parent[root_v] = root_u as isize;
        siz[root_u] += siz[root_v];
    }
}

fn is_same(u: usize, v: usize, parent: &Vec<isize>) -> bool {
    root(u, parent) == root(v, parent)
}
