use proconio::input;

fn main(){
    input!{
        n:usize,
        mut arr:[usize;n],
    };
    let mut pq = vec![(0,0);n];
    for i in 0..n{
        let mut prime = 2;
        while prime * prime <= arr[i]{
            if arr[i] % (prime * prime)==0{
                arr[i] /= prime*prime;
                pq[i].0 = prime;
                pq[i].1 = arr[i];
                break;
            }else if arr[i] % prime == 0 {
                arr[i] /= prime;
                pq[i].1 = prime;
                pq[i].0 = (arr[i] as f64).sqrt().round() as usize;
                break;
            }else{
                prime += 1;
            }
        }
    }
    for i in 0..n{
        println!("{} {}",pq[i].0,pq[i].1);
    }
}