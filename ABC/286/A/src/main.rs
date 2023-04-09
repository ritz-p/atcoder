use proconio::input;

fn main(){
    input!{
        n: usize,
        p: usize,
        q: usize,
        r: usize,
        s: usize,
        mut arr: [usize;n],
    };
    let diff = r-p;
    for i in 0..n{
        if i+1 >= p && i+1 <= q{
            let tmp = arr[i];
            arr[i] = arr[i+diff];
            arr[i+diff] = tmp;
        }
    }
    // println!("{:?}",arr);
    for i in 0..n{
        print!("{} ",arr[i]);
    }
}