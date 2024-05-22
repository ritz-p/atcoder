use proconio::input;

fn main(){
    input!{
        n: usize,
        m: usize,
        ab: [(usize,usize);m]
    };
    let mut res = 1;
    let mut graph = vec![vec![];n];
    for (a,b) in ab{
        graph[a-1].push(b-1);
        graph[b-1].push(a-1);
    }
    let mut current = 0;
    let mut visited = vec![false;n];
    let mut colors = vec![-1;n];
    for i in 0..n{
        if visited[i]{
            break;
        }
        res *= 3;
        let mut points = vec![];
        colors[points[0]] = 0;
        let mut current = 0;

        dfs(&graph,&mut visited,current,&mut points);
    }
    loop{
        if visited[current]{
            current += 1;
            if current == n{
                break;
            }
            continue;
        }

        res *= 3
    }
    println!("{}",res);
}

fn dfs(graph: &Vec<Vec<usize>>,visited: &mut Vec<bool>,index: usize,points: &mut Vec<usize>){
    if visited[index]{
        return;
    }
    visited[index] = true;
    points.push(index);
    for &g in &graph[index]{
        dfs(graph,visited,g,points);
    }
}

fn dfs2(index: usize,colors: Vec<isize>,current: &mut usize,points: Vec<usize>){
    if index == points.len(){
        *current += 1;
        return;
    }
    let v = points[index];
    for i in 0..3{
        colors[v] = i;
        let mut ok = true;
        for u in 
    }
}
