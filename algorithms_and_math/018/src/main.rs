use proconio::input;

fn main(){
    input!{
        n: i32,
        arr: [i32;n],
    };
    let mut a:i64 = 0;
    let mut b:i64 = 0;
    let mut c:i64 = 0;
    let mut d:i64 = 0;
    for i in arr{
        if i==100{a += 1;continue;}
        if i==200{b += 1;continue;}
        if i==300{c += 1;continue;}
        if i==400{d += 1;continue;}
    }
    let res = a * d + b * c;
    println!("{}",res);
}