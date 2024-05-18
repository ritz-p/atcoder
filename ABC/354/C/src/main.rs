use proconio::input;
use std::collections::BTreeMap;
use std::collections::HashSet;
fn main(){
    input!{
        n: usize,
        mut ac: [(usize,usize);n]
    };

    let mut bmap = BTreeMap::new();
    let mut v:Vec<(usize,&usize,&usize)> = vec![];
    for (index,(a,c)) in ac.iter().enumerate(){
        bmap.insert(*a,index);
        v.push((index,a,c));
    }
    v.sort_by(|a,b|a.2.cmp(&b.2));
    let mut remove:HashSet<usize> = HashSet::new();
    for (index,a,_c) in v.iter().rev(){
        if let Some(v) = bmap.last_key_value(){
            if v.0 > &a{
                remove.insert(*index);
            }
            bmap.remove(a);
        }
    }
    println!("{}",n-remove.len());
    v.sort_by(|a,b|a.0.cmp(&b.0));
    for i in 0..n{
        if !remove.contains(&v[i].0){
            print!("{} ",v[i].0+1);
        }
    }
}
