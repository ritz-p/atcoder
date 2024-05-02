use itertools::Itertools;
use proconio::input;
fn main(){
    input!{
        n: usize,
        m: usize,
        ab: [(usize,usize);m]
    };
    let mut graph = vec![vec![];n];

    for (a,b) in ab{
        graph[a-1].push(b-1);
        graph[b-1].push(a-1);
    }
    let mut visited = vec![false;n];
    let mut route = vec![];
    dfs(&graph,&mut visited,&mut route,0);
    println!("{}",route.iter().join(" "));
}

fn dfs(graph: &Vec<Vec<usize>>,visited: &mut Vec<bool>,route: &mut Vec<usize>,index: usize){
    visited[index] = true;
    route.push(index+1);
    if index == graph.len()-1{
        println!("{}",route.iter().join(" "));
        return;
    }
    for i in &graph[index]{
        if visited[*i]{
            continue;
        }
        dfs(graph,visited,route,*i);
    }
    route.pop();
}