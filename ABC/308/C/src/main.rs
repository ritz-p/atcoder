use proconio::input;
 
fn main() {
    input! {
        n: usize,
        mut ab: [(i64,i64);n]
    }
    let mut c = vec![];
    for (idx, &v) in ab.iter().enumerate() {
        c.push((v.0,v.1,idx+1));
    }
    c.sort_by(|a,b| ((b.0*(a.0+a.1)).cmp(&(a.0*(b.0+b.1)))));
    for i in 0..n {
        print!("{} ",c[i].2);
    }
    println!();
}