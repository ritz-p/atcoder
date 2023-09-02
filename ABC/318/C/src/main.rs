use proconio::input;

fn main(){
    input!{
        n: usize,
        d: usize,
        p: usize,
        mut f: [usize;n]
    };
    f.sort_by(|a, b| b.cmp(&a));
    let mut sum = f.iter().fold(0, |sum, x| sum + x);
    let mut current_position = 0;
    loop{
        let mut instant_sum = 0;
        for i in current_position..current_position+d{
            if i < f.len(){
                instant_sum += f[i];
            }
        }
        // println!("{}",instant_sum);
        if instant_sum > p{
            sum -= instant_sum;
            sum += p;
            current_position += d;
            // println!("{} {}",instant_sum,current_position);
        }else{
            break;
        }
    }
    println!("{}",sum);
}
