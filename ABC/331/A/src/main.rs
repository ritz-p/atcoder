use proconio::input;

fn main(){
    input!{
        m: usize,
        d: usize,
        y: usize,
        mm: usize,
        dd: usize
    };

    let (next_year, next_month, next_day) = next_date(y, mm, dd, m, d);
    println!("{} {} {}",next_year,next_month,next_day);
}
fn next_date(y: usize, mm: usize, dd: usize, m: usize, d: usize) -> (usize, usize, usize) {
    if dd < d {
        return (y, mm, dd + 1);
    } else {
        if mm < m {
            return (y, mm + 1, 1);
        } else {
            return (y + 1, 1, 1);
        }
    }
}