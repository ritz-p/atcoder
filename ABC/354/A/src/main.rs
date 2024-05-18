use proconio::input;

fn main(){
    input!{
        h: usize,
    };
    let mut p = 0;
    let mut c = 0;
    loop{
        c += 2_usize.pow(p);
        if c > h{
            println!("{}",p+1);
            return;
        }
        p += 1;
    }
}
