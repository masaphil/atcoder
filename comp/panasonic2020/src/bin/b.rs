use proconio::input;

fn main() {
    input!(h: usize, w: usize);
    let mut ans: usize = 0;
    let ex: usize = h * w;
    if h == 1 || w == 1 {
        println!("{}", 1)
    } else {
        match ex % 2 {
            1 => {
                ans = (ex + 1) / 2;
                println!("{}", ans);
            }
            0 => {
                ans = ex / 2;
                println!("{}", ans);
            }
            _ => panic!(),
        }
    }
}
