use proconio::input;

fn dfs(map: &Vec<Vec<usize>>, mut visited: Vec<bool>, mut searched: Vec<bool>, mut ans: usize, key: usize) -> (usize, Vec<bool>, Vec<bool>) {
    if visited[key] {
        return (ans, visited, searched);
    }

    visited[key] = true;
    searched[key] = true;

    for &k in map[key].iter() {

        if searched[k] {
            continue;
        }

        if visited[k] {
            ans += 1;
            continue;
        }

        let (a, v, s) = dfs(map, visited, searched, ans, k);
        ans = a;
        visited = v;
        searched = s;
    }

    searched[key] = false;
    (ans, visited, searched)
}

fn main() {
    input!{
        n: usize,
        m: usize,
    };

    let mut visited = vec![false; n];
    let mut searched = vec![false; n];
    let mut map: Vec<Vec<usize>> = vec![vec![]; n];

    for _ in 0..m {
        input!{
            a: usize,
            b: usize,
        };

        let a = a-1;
        let b = b-1;

        map[a].push(b);
        map[b].push(a);
    }

    let mut ans = 0;
    for i in 0..n {
        if !visited[i] {
            let (a, v, s) = dfs(&map, visited, searched, ans, i);
            ans = a;
            visited = v;
            searched = s;
        }
    }

    println!("{}", ans);
}
