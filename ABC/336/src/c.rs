use proconio::input;

fn main(){
    input!{
        mut n: usize
    };
    let mut s = base(n-1,5).to_string();
    s = s.replace("4","8");
    s = s.replace("3","6");
    s = s.replace("2","4");
    s = s.replace("1","2");
    println!("{}",s);
}

fn base(mut x: usize,b: usize)->usize{
    let mut remainder: Vec<usize> = Vec::new();
    while x != 0{
        remainder.push(x % b);
        x /= b;
    }
    let mut n:usize = 0;
    for i in 0..remainder.len(){
         n += 10usize.pow(i as u32)*remainder[i]
    }
    return n;
}