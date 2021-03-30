use proconio::input;

fn main() {
    input! {
        n:u32,
        d:[u32;n],
    };
    let mut ans = 0;
    let min = *d.iter().min().unwrap();
    let max = *d.iter().max().unwrap();
    for k in min..=max {
        let mut cntr = 0;
        for i in &d {
            if &k <= i {
                cntr += 1;
            }
        }
        if cntr == n - cntr {
            ans += 1;
        }
    }
    println!("{}", ans);
}
