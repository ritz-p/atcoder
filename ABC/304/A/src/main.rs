use proconio::input;

fn main(){
    input!{
        n: usize,
        mut arr: [(String,usize);n]
    };
    let mut min = arr[0].1;
    let mut p = 0;
    for i in 1..n{
        if min > arr[i].1{
            min = arr[i].1;
            p = i;
        }
    }
    for i in 0..n{
        if i + p < n{
            println!("{}",arr[i+p].0);
        }else{
            println!("{}",arr[i+p-n].0);
        }
    }
}
