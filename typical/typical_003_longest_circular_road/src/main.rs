use proconio::input;
fn main() {
    input!{
        n: usize,
        r: [(usize,usize);n-1]
    }
    let mut roads = vec![vec![];n+1];
    for (x,y) in r.iter(){
        roads[*x].push(*y);
        roads[*y].push(*x);
    }
    let mut ans = 0;
    dfs(&roads,1,0,&mut ans);

    println!("{}",ans);
}

fn dfs(roads: &Vec<Vec<usize>>,index: usize,pre: usize,ans: &mut usize) -> usize{
    let mut v = vec![];
    for &next in roads[index].iter(){
        if next == pre{
            continue;
        }
        v.push(dfs(roads,next,index,ans));
    }
    if v.is_empty(){
        1
    }else{
        if v.len() >= 2{
            v.sort_by(|a,b| b.cmp(&a));
            *ans = (*ans).max(v[0] + v[1] + 1);
        }else{
            *ans = (*ans).max(v[0] + 1);
        }
        v[0] + 1
    }
}
