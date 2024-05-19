use proconio::input;
use std::collections::HashSet;
fn main(){
    input!{
        n: usize,
        tup: [(usize,usize);n-1]
    };
    let mut graph = vec![vec![];n+1];
    for i in 0..n-1{
        graph[tup[i].0].push(tup[i].1);
        graph[tup[i].1].push(tup[i].0);
    }
    let mut nodes = vec![];
    for i in &graph[1]{
        let mut visited = HashSet::new();
        dfs(&graph,*i,&mut visited);
        nodes.push(visited.len());
    }
    let mut res = 0;
    nodes.sort();
    for i in 0..graph[1].len()-1{
        res += nodes[i];
    }
    res += 1;
    println!("{}",res);
}

fn dfs(graph: &Vec<Vec<usize>>,index: usize,visited: &mut HashSet<usize>){
    if visited.contains(&index) || index == 1{
        return;
    }
    visited.insert(index);
    for i in &graph[index]{
        dfs(graph,*i,visited);
    }
}