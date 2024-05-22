use proconio::input;

fn main(){
    input!{
        n: usize,
        a: [usize;n],
        b: [usize;n],
    };
    let am = a.iter().max().unwrap();
    let bm = b.iter().min().unwrap();
    if am > bm{
        println!("0");
    }else{
        println!("{}",bm-am+1);
    }
}
