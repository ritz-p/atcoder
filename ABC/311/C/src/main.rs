use proconio::input;
fn main(){
    input!{
        n: usize,
        arr: [usize;n]
    };
    let mut graph:Vec<Vec<usize>> = vec![vec![];n+1];
    let mut closed = false;
    for i in 1..n{
        graph[i+1].push(arr[i]);
    }
    let mut visited = vec![false;n+1];
    for i in 0..n{
        let mut res = vec![];
        if !visited[i]{
            dfs(&graph,&mut visited,&i,&mut closed,&i,&mut res);
        }
        if closed{
            return
        }
    }
}


fn dfs(graph: &Vec<Vec<usize>>,visited:&mut Vec<bool>,index: &usize,closed:&mut bool,start: &usize,res: &mut Vec<usize>){
    if visited[*index]{
        return;
    }
    visited[*index] = true;
    for i in &graph[*index]{
        if i == start{
            *closed = true;
            let mut current = start;
            res.push(*current);
            loop{
                if current == index{
                    break;
                }
                res.push(graph[*current][0]);
                current = &graph[*current][0];
            }
            println!("{}",res.len());
            for i in res{
                println!("{} ",i);
            }
            return;
        }
        dfs(graph,visited,&i,closed,start,res);
    }
}

