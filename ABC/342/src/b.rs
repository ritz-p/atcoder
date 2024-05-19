use proconio::input;

fn main(){
    input!{
        n: usize,
        p: [usize;n],
        q: usize,
        ab: [(usize,usize);q]
    };
    let mut sorted = vec![0;n];
    for i in 0..n{
        sorted[p[i]-1] = i;
    }
    for (a,b) in &ab{
        if sorted[*a-1] > sorted[*b-1]{
            println!("{}",p[sorted[*b-1]]);
        }else{
            println!("{}",p[sorted[*a-1]]);
        }
    }
}
