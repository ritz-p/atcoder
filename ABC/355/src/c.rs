use proconio::input;
use std::collections::HashMap;
fn main(){
    input!{
        n: usize,
        t: usize,
        a: [usize;t]
    };
    let mut graph = vec![vec![0;n];n];
    let mut vertical = vec![0;n];
    let mut horizontal = vec![0;n];
    let mut diagonal = vec![0;2];
    let mut map = HashMap::new();
    let v:usize = (1..n).sum();
    let h:usize = (1..=n).sum();
    for i in 0..n{
        vertical[i] = n * (i+1) + n * v;
        horizontal[i] = h + n * n * i;
    }
    for i in 0..n{
        for j in 0..n{
            graph[i][j] = n * i + j + 1;
            map.insert(graph[i][j],(i,j,3));
            if i == j{
                diagonal[0] += graph[i][j];
                map.insert(graph[i][j],(i,j,0));
            }
            if i + j == n - 1{
                diagonal[1] += graph[i][j];
                map.insert(graph[i][j],(i,j,1));
            }
            if i == j && i + j == n - 1{
                map.insert(graph[i][j],(i,j,2));
            }
        }
    }
    for (index,e) in a.iter().enumerate(){
        if let Some(value) = map.get(&e){
            vertical[value.1] -= e;
            horizontal[value.0] -= e;
            match value.2{
                0 => {
                    diagonal[value.2] -= e;
                },
                1 => {
                    diagonal[value.2] -= e;
                },
                2 => {
                    diagonal[0] -= e;
                    diagonal[1] -= e;
                },
                _ => {}
            }
            if vertical[value.1] == 0 || horizontal[value.0] == 0 || diagonal[0] == 0 || diagonal[1] == 0{
                println!("{}",index+1);
                return;
            }
        }
    }
    println!("-1");
}
