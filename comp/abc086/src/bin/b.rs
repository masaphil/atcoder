use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(a: u32, b: u32);
    let mut ans: bool = false;
    let num: u32;
    if b == 100 {
        num = a * 1000 + b
    } else if 10 <= b && b <= 99 {
        num = a * 100 + b
    } else {
        num = a * 10 + b
    }
    for i in 0..=1000 {
        if i * i == num {
            ans = true;
        }
    }
    if ans {
        println!("Yes")
    } else {
        println!("No")
    }
}
