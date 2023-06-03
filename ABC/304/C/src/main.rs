use proconio::input;
use std::collections::HashSet;
fn main(){
    input!{
        n: usize,
        d: usize,
        xy: [(isize,isize);n]
    };
    let mut flag = vec![false;n];
    flag[0] = true;
    let mut affected = HashSet::new();
    loop {
        let current = affected.len();
        for i in 0..n{
            if flag[i] && !affected.contains(&i){
                for j in 0..n{
                    if i != j{
                        let distance = ((xy[i].0-xy[j].0)*(xy[i].0-xy[j].0)) as usize + ((xy[i].1-xy[j].1)*(xy[i].1-xy[j].1)) as usize;
                        if (distance as f64).sqrt() <= d as f64{
                            flag[j] = true;
                        }
                    }
                }
                affected.insert(i);
            }
        }
        if current == affected.len(){
            break;
        }
    }
    for i in 0..n{
        if flag[i]{
            println!("Yes");
        }else{
            println!("No");
        }
    }
}
