use proconio::input;
use std::collections::HashMap;
fn main(){
    input!{
        n: usize,
        a: [isize;n]
    };

    let mut row:Vec<usize> = vec![];
    let mut pair = HashMap::new();
    for i in 0..n{
        if a[i] == -1{
            row.push(i+1);
        }else{
            pair.insert(a[i],i+1);
        }
    }
    dfs(row[0],pair,&mut row);
    for i in row{
        print!("{} ",i);
    }
}

fn dfs(current: usize,pair: HashMap<isize,usize>,row:&mut Vec<usize>){
    let next = pair.get(&(current as isize));
    match next{
        Some(i) => {
            row.push(*i);
            dfs(*i,pair,row);
        },
        None => {}
    }
}