use proconio::input;
use itertools::Itertools;
fn main(){
    input!{
        n: usize,
        m: usize,
        arr: [(usize,usize);m],
    }
    let mut data: Vec<Vec<usize>> = vec![];
    for i in 0..n {
        let mut v: Vec<usize> = vec![];
        data.push(v);
    }
    for pair in arr{
        let i1 = pair.0;
        let i2 = pair.1;
        data[(i1-1)].push(i2);
        data[(i2-1)].push(i1);
    }
 
    for mut row in data{
        row.sort();
        // println!("{:?}", row);
        let mut ans = vec![row.len()];
        ans.append(&mut row);
        println!("{}", ans.iter().join(" "));
    }

}