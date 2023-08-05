use proconio::input;

fn main(){
    input!{
        n: usize,
        arr: [usize;n]
    };
    let mut max = arr[0];
    let mut another = false;
    for i in 1..n{
        if arr[i] > max{
            max = arr[i];
            another = false;
        }else if arr[i] == max{
            another = true;
        }
    }
    if arr[0] == max && !another{
        println!("0");
    }else if arr[0] == max && another{
        println!("{}",1);
    }else{
        println!("{}",max+1-arr[0]);
    }
}
