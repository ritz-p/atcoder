use proconio::input;

fn main(){
    input!{
        n: usize,
        m: usize,
        tup: [(usize,usize);m]
    };
    let mut a = vec![vec![];n+1];
    let mut b = vec![vec![];n+1];

    for i in 0..m{
        a[tup[i].0].push(tup[i].1);
        b[tup[i].1].push(tup[i].0);
    }
    let a_cl = a.clone();

    for i in 1..n{
        for j in &a_cl[i]{
            dfs(&mut a,&a_cl,i,*j);
        }
    }
    for i in 1..n{
        if a[i].len() == n-1{
            println!("{}",i+1);
            return;
        }
    }
    println!("{:?}",a);
    println!("-1");
}

fn dfs(graph: &mut Vec<Vec<usize>>,graph_cl: &Vec<Vec<usize>>,base: usize,index: usize){
    if graph[index].len() == 0{
        return
    }else{
        for g in 0..graph[index].len(){
            graph[base].push(graph_cl[index][g]);
            dfs(graph,graph_cl,base,graph[index][g]);
        }
    }
}
