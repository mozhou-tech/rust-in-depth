use std::ptr::write;

fn main() {
    let mut  s1 = "Hello Rust.";  // 变量绑定
    s1 = "xxasdfasdf";
    for c in s1.chars() {
        println!("{}",&c)
    }
    println!("printing {}", &s1);
}
