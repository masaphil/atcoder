fn main() {
    proconio::input! {
    n: usize,
    mut v: [u32; n],
    }
    let mut swt: bool = true;
    let mut ali: u32 = 0;
    let mut bob: u32 = 0;
    v.sort_by(|a, b| b.partial_cmp(a).unwrap());
    for i in v {
        if swt {
            ali += i;
            swt = false;
        } else {
            bob += i;
            swt = true
        }
    }
    println!("{}", ali - bob);
}
