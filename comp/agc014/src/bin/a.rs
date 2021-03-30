use proconio::input;

fn main() {
    input! {
        mut a:usize,
        mut b:usize,
        mut c:usize,
    };
    let mut ans = 0;
    let mut spa;
    let mut spb;
    let mut spc;
    while a % 2 == 0 && b % 2 == 0 && c % 2 == 0 {
        spa = a / 2;
        spb = b / 2;
        spc = c / 2;
        a = spb + spc;
        b = spa + spc;
        c = spa + spb;
        if a == b && b == c {
            println!("-1");
            return;
        }
        ans += 1;
    }
    println!("{}", ans);
}
