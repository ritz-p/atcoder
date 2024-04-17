use proconio::input;

fn main(){
    input!{
        n: usize,
        q: [isize;n],
        a: [isize;n],
        b: [isize;n]
    };
    let m = q.iter().max().unwrap();
    let mut res = 0;
    for i in 0..(m+1)as usize{
        let mut j = isize::MAX;
        for k in 0..n{
            if a[k] * i as isize > q[k]{
                j = isize::MIN;
            }else if b[k] > 0{
                j = j.min((q[k] - a[k] * i as isize) / b[k]);
            }
        }
        res = res.max(i as isize+j);
    }
    println!("{}",res);
}
