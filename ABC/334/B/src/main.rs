use proconio::input;

fn main(){
    input!{
        a: isize,
        m: isize,
        mut l: isize,
        mut r: isize,
    };
    let x = (l - 1 - a).div_euclid(m);
    let y = (r - a).div_euclid(m);
    println!("{}", y - x);
}
