use std::collections::VecDeque;

fn main()
{
    proconio::input! {
        n: usize,
        m: usize,
        xy: [(usize, usize); m],
    }

    let mut g = vec![vec![]; n];
    let mut cnt = vec![0; n];
    for (x, y) in xy {
        g[x - 1].push(y - 1);
        cnt[y - 1] += 1;
    }

    let mut q = VecDeque::new();
    for i in 0..n {
        if cnt[i] == 0 {
            q.push_back(i);
        }
    }

    let mut dag = Vec::new();
    while let Some(u) = q.pop_front() {
        dag.push(u);
        for v in &g[u] {
            cnt[*v] -= 1;
            if cnt[*v] == 0 {
                q.push_back(*v);
            }
        }
    }

    for i in 0..(n - 1) {
        let u = dag[i];
        let v = dag[i + 1];
        if g[u].contains(&v) {
            continue;
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
    let mut ans = vec![0; n];
    for i in 0..n {
        ans[dag[i]] = i;
    }
    for u in ans {
        print!("{} ", u + 1);
    }
}