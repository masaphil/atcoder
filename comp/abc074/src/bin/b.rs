use proconio::input;

fn main() {
    input! {n:u32,k:u32,x:[u32;n]};
    let mut an: u32 = 0;
    for i in x {
        an += if i < k - i { i } else { k - i };
    }
    println!("{}", an * 2)
}
