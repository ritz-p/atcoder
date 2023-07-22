use proconio::input;
use std::collections::HashSet;

fn main(){
    input!{
        n: usize,
        arr: [usize;n]
    };
    let mut graph:Vec<HashSet<usize>> = vec![HashSet::new();n+1];
    let mut closed = false;
    for i in 0..n{
        graph[i+1].insert(arr[i]);
    }
    // println!("{:?}",graph);
    let mut start_at = 1;
    for i in 1..n{
        let mut visited = vec![false;n+1];
        dfs(&graph,&mut visited,&i,&mut closed,&mut start_at);
        // println!("{:?}",visited);
        if closed{
            let mut visited = vec![false;n+1];
            let mut garbage = 0;
            let mut count = 0;
            dfs(&graph,&mut visited,&mut start_at,&mut closed,&mut garbage);
            for i in 1..visited.len(){
                if visited[i]{
                    count += 1;
                }
            }
            println!("{}",count);
            for _ in 0..count{
                print!("{} ",start_at);
                for g in &graph[start_at]{
                    start_at = *g;
                }
            }
            return
        }
    }
}


fn dfs(graph: &Vec<HashSet<usize>>,visited:&mut Vec<bool>,index: &usize,closed:&mut bool,start_at:&mut usize){
    if graph[*index].len() == 0{
        return
    }
    if visited[*index]{
        *closed = true;
        *start_at = *index;
        return;
    }
    visited[*index] = true;
    for g in &graph[*index]{
        dfs(graph,visited,&g,closed,start_at);
    }
}

