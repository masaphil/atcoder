use proconio::input;

fn main() {
    input! {n:u32, a:[u32;n]};
    let mut push = a[0] - 1;
    let mut cnt = 1;
    let mut ans = true;
    let mut prime = true;
    if push == 1 {
        println!("1");
        prime = false;
    }
    if prime {
        for _ in 0..n {
            cnt += 1;
            push = a[push as usize] - 1;
            if push == 1 {
                println!("{}", cnt);
                ans = false;
                break;
            }
        }
        if ans {
            println!("-1")
        }
    }
}
