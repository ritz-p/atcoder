use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize,usize);m],
        cd: [(usize,usize);m]
    };
    let mut ab_graph = vec![vec![false;n];n];
    let mut cd_graph = vec![vec![false;n];n];

    for (a,b) in ab{
        ab_graph[a-1][b-1] = true;
        ab_graph[b-1][a-1] = true;
    }
    
    for (c,d) in cd{
        cd_graph[c-1][d-1] = true;
        cd_graph[d-1][c-1] = true;
    }

    for perm in (0..n).permutations(n){
        let mut flag = true;
        for i in 0..n{
            for j in 0..n{
                if ab_graph[i][j] != cd_graph[perm[i]][perm[j]]{
                    flag = false;
                }
            }
        }
        if flag{
            println!("Yes");
            return;
        }
    }
    println!("No");
}
