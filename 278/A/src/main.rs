use proconio::input;

fn main(){
    input!{
        n: usize,
        k: usize,
        mut arr: [usize;n],
    };

    for _i in 0..k{
        arr.remove(0);
        arr.push(0);
    }

    for i in 0..n-1{
        print!("{} ",arr[i]);
    }
    print!("{}\n",arr[n-1]);
}