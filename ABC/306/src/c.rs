use proconio::input;

fn main(){
    input!{
        n: usize,
        arr: [usize;n*3]
    };
    let mut tup:Vec<(usize,usize)> = vec![];
    let mut count = vec![0;n];
    for i in 0..n*3{
        count[arr[i]-1] += 1;
        if count[arr[i]-1] == 2{
            tup.push((arr[i],i));
        }
    }
    tup.sort_by(|a,b| a.1.partial_cmp(&(b.1)).unwrap());
    for i in tup{
        print!("{} ",i.0);
    }
}
