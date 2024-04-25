use proconio::input;
use std::collections::HashSet;
fn main(){
    input!{
        n: usize,
        m: usize,
    };
    let mut vecset:Vec<HashSet<usize>> = vec![HashSet::new();n];
    let mut res = 0;
    let mut visited = vec![false;n];
    for _i in 0..m{
        input!{
            a: usize,
            b: usize,
        };
        vecset[a].insert(b);
        vecset[b].insert(a);
    }
    for i in 0..n{
        let current = vecset[i];
        for j in current{
            for k in &vecset[j]{
                if !vecset[j].contains(k){
                    vecset[j].insert(*k);
                }
            }
        }
    }
}

fn bfs(visited: Vec<bool>,mut vecset:Vec<HashSet<usize>>,index: usize){

}