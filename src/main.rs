

fn main(){
    let s1 = "hello";
    let s2 = "world";
    let mut s3 = format!("{} {}", s1, s2);

    for c in s3.chars() {
        println!("{}", c);
    }

    s3.push('!');
    s3.push_str(" 2026");

    println!("{}", s3);
}