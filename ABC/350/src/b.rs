use proconio::input;

fn main(){
    input!{
        n : usize,
        q: usize,
        t: [usize;q]
    };
    let mut v = vec![true;n];
    for i in t{
        if v[i-1]{
            v[i-1] = false;
        }else{
            v[i-1] = true;
        }
    }
    println!("{}",v.iter().filter(|&&x| x).count());
}
