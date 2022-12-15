use proconio::input;
use std::cmp::max;


fn rec(graph: &[Vec<usize>],dp:&mut Vec<isize>,index:usize) -> isize{
    if dp[index] != -1{
        return dp[index]
    }
    let mut res:isize = 0;
    for i in graph[index].iter(){
        res = max(res,rec(graph,dp,*i)+1);
    }
    dp[index] = res;
    res
}
fn main(){
    input!{
        n: usize,
        m: usize,
        bars: [(usize,usize);m],
    };
    let mut graph = vec![vec![];n+1];
    let mut dp:Vec<isize> = vec![-1;n+1];

    for i in 0..m{
        graph[bars[i].0].push(bars[i].1);
    }
    let mut res:isize = 0;
    for i in 0..n+1{
        res = max(res,rec(&graph,&mut dp,i));
    }

    // println!("{:?}",graph);
    println!("{}",res);
}