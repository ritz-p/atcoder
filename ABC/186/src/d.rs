use proconio::input;
fn main(){
    input!{
        n: usize,
        mut a: [isize;n]
    };
    let mut res = 0;
    a.sort();
    let mut p = 0;
    for (index,e) in a.iter().enumerate(){
        if *e >= 0{
            p = index;
            break;
        }
    }
    println!("{}",p);
    for (index,v) in a.iter().rev().enumerate(){
        if v < &0{
            res += v.abs() * (n - 1 - index) as isize;
            res -= v.abs() * (n - p) as isize;
        }else{
            res += v.abs() * (n - 1 - index) as isize;
            res -= v.abs() * p as isize;
        }
        println!("{} {} {}",res,index,v);
    }
    println!("{}",res);
}
