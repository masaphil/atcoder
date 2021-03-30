use proconio::input;

fn main() {
    input! {
        k:u32,
        n:u32,
        a:[u32;n]
    };
    let mut dis = a[0 as usize] + k - a[(n - 1) as usize];
    let mut d: u32;
    for i in 1..n {
        d = a[i as usize] - a[(i - 1) as usize];
        dis = dis.max(d);
    }
    println!("{}", k - dis);
}
