use proconio::input;
use std::collections::HashSet;
fn main(){
    input!{
        n: usize,
        m: usize,
        v: [[isize;4];m]
    };
    let mut pos:Vec<(isize,isize)> = vec![(100001,100001);n];
    pos[0] = (0,0);
    let mut relation:Vec<HashSet<(usize,isize,isize)>> = vec![HashSet::new();n];
    for i in 0..m{
        relation[v[i][0] as usize -1].insert((v[i][1] as usize -1,v[i][2],v[i][3]));
        relation[v[i][1] as usize -1].insert((v[i][0] as usize -1,v[i][2]*(-1),v[i][3]*(-1)));
    }
    let mut visited = vec![false;n];
    dfs(&mut pos,&relation,0,&mut visited);
    for i in 0..n{
        if pos[i].0 == 100001 && pos[i].1 == 100001{
            println!("undecidable");
        }else{
            println!("{} {}",pos[i].0,pos[i].1);
        }
    }
}

fn dfs(pos:&mut Vec<(isize,isize)>,relation: &Vec<HashSet<(usize,isize,isize)>>,index: usize,visited:&mut Vec<bool>){
    if relation[index].len() == 0 || visited[index]{
        return;
    }
    visited[index] = true;
    for (x,y,z) in &relation[index]{
        pos[*x].0 = pos[index].0 + *y;
        pos[*x].1 = pos[index].1 + *z;
        dfs(pos,relation,*x,visited);
    }
}