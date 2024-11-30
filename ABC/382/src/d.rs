use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let mut current = vec![];
    let mut res = vec![];
    dfs(0, n, m, &mut current, &mut res);
    println!("{}", res.len());
    for e in res {
        println!("{}", e.iter().join(" "));
    }
}

fn dfs(pos: usize, n: usize, m: usize, current: &mut Vec<usize>, res: &mut Vec<Vec<usize>>) {
    if pos == n {
        res.push(current.to_vec());
        return;
    }
    let min = if pos == 0 {
        1
    } else {
        current.last().unwrap() + 10
    };
    let max = m - 10 * (n - pos - 1);
    if min > max {
        return;
    }
    for i in min..=max {
        current.push(i);
        dfs(pos + 1, n, m, current, res);
        current.pop();
    }
}
