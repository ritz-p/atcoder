use proconio::*;
fn main(){
    input!{
        z:[(i32,i64)]
    }
    let mut d=0;
    let mut e=-10i64.pow(18);
    for&(x,y)in&z{
        if x<1{
            d=d.max(d.max(e)+y);
        }
        else{
            e=e.max(d+y);
        }
    }
    println!("{}",d.max(e))
}
