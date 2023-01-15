use proconio::input;
use std::collections::HashMap;
fn main(){
    input!{
        n: usize,
        arr: [(String,String);n],
    };
    let mut hash = HashMap::new();
    let mut flag = vec![false;n];
    for i in 0..n{
        hash.insert(&arr[i].0,(&arr[i].1));
    }
    dfs(hash,arr[0].0);
    println!("{:?}",hash);
}

fn dfs(mut hash: HashMap<&String,(&String,bool)>,key: String,mut flag: Vec<bool>,index:usize) -> bool{
    if hash.contains_key(&key){
        if flag[index] == false{
            flag[index] = true;
            dfs(hash,pair.0);
        }
    }
}