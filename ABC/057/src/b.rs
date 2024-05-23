use proconio::input;

fn main(){
    input!{
        n: usize,
        m: usize,
        ab: [(isize,isize);n],
        cd: [(isize,isize);m]
    };

    for (a,b) in ab{
        let mut pos = 0;
        let mut current = isize::MAX;
        for (index,(c,d)) in cd.iter().enumerate(){
            if (a-c).abs() + (b-d).abs() < current{
                pos = index;
                current = (a-c).abs() + (b-d).abs();
            }
        }
        println!("{}",pos+1);
    }
}
