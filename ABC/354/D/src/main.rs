use proconio::input;

fn main(){
    input!{
        a: isize,
        b: isize,
        c: isize,
        d: isize,
    };
    let m = 1000000000;
    let a = (a + m) as usize;
    let b = (b + m) as usize;
    let c = (c + m) as usize;
    let d = (d + m) as usize;


    println!("{}",f(c,d) + f(a,b) - f(c,b) - f(a,d));
}

fn f(x: usize,y: usize) -> usize{
    let arr = vec![vec![0,0,0,0,0],vec![0,2,3,3,4],vec![0,3,6,7,8]];
    let s1 = (y / 2) * (x / 4) * arr[2][4];
    let s2 = (y / 2) * arr[2][x % 4];
    let s3 = (x / 4) * arr[y % 2][4];
    let s4 = arr[y % 2][x % 4];

    s1 + s2 + s3 + s4
}