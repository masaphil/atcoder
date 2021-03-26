use proconio::input;

fn main() {
    input!(n: u32, a: u32, b: u32, st: String);
    // nのイテレータのfor文
    let mut cnt = 0;
    let mut cntb = 0;
    for i in st.chars() {
        match i {
            'a' => {
                if a + b > cnt {
                    println!("Yes");
                    cnt += 1;
                } else {
                    println!("No")
                }
            }
            'b' => {
                cntb += 1;
                if a + b > cnt && cntb <= b {
                    println!("Yes");
                    cnt += 1;
                } else {
                    println!("No")
                }
            }
            'c' => {
                println!("No")
            }
            _ => unreachable!(),
        }
    }
}
