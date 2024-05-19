use proconio::input;
use std::cmp::max;
fn main(){
    input!{
        n: usize,
        m: usize,
    };

    let mut road:Vec<Vec<(usize,usize)>> = vec![vec![];n];
    for _i in 0..m{
        input!{
            a: usize,
            b: usize,
            c: usize,
        }
        road[a-1].push((b,c));
        road[b-1].push((a,c));
    }
    let mut max_len = 0;
    for i in 0..n{
        let mut visited = vec![false;n];
        max_len = max(max_len,dfs(&road,&mut visited,i));
    }
    // println!("{:?}",road);
    println!("{}",max_len);
}

fn dfs(graph: &Vec<Vec<(usize,usize)>>, visited: &mut Vec<bool>,index: usize) -> usize{
    visited[index] = true;
    let mut max_len = 0;
    for element in &graph[index]{
        if !visited[element.0-1]{
            max_len = max(max_len,element.1 + dfs(graph,visited,element.0-1));
        }
    }
    visited[index] = false;
    max_len
}