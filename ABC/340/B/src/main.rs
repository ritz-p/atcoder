use proconio::input;

fn main(){
    input!{
        q: usize,
        kx: [(usize,usize);q]
    };
    let mut a = vec![];
    for (k,x) in kx{
        if k == 1{
            a.push(x);
        }else if k == 2{
            println!("{}",a[a.len()-x]);
        }
    }
}
