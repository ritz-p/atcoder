use proconio::input;
use std::collections::BTreeMap;
fn main(){
    input!{
        n: usize,
        m: usize,
        a: [usize;n],
        mut b: [usize;m]
    };

    let mut amap = BTreeMap::new();
    for i in a{
        *amap.entry(i).or_insert(0) += 1;
    }
    b.sort();
    let mut res = 0;
    for e in b{
        let mut current = amap.range(e..);
        if let Some((&key,_value)) = current.next(){
            res += key;
            if let Some(v) = amap.get_mut(&key){
                *v -= 1;
                if *v == 0{
                    amap.remove(&key);
                }
            }
        }else{
            println!("-1");
            return;
        }
    }
    println!("{}",res);
}
