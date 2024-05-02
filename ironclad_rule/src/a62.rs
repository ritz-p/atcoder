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
    dfs(&graph,&mut visited,0);
    if visited.iter().any(|f| *f == false){
        println!("The graph is not connected.");
    }else{
        println!("The graph is connected.");
    }
}

fn dfs(graph: &Vec<Vec<usize>>,visited: &mut Vec<bool>,index: usize){
    if visited[index]{
        return;
    }
    visited[index] = true;
    for i in &graph[index]{
        dfs(graph,visited,*i);
    }
}