use proconio::input;

fn main(){
    input!{
        n: usize,
        mut m: usize,
        h: [usize;n]
    };
    for (index,i) in h.iter().enumerate(){
        if m < *i{
            println!("{}",index);
            return;
        }else{
            m -= *i;
        }
        if index == n-1{
            println!("{}",index+1);
        }
    }
}
