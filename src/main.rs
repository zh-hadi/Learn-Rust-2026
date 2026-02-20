



fn main() {
    let mut big_number:Result<_,i32> = Ok(0);
    let data = [10, 20, 86, 2, 50, 20];

    for i in 0..6 {
        if data[i] > big_number.unwrap() {
            big_number = Ok(data[i])
        }
        println!("{}", data[i]);
    }

    println!("big number is {}", big_number.unwrap());

    match big_number {
        Ok(v) => println!("big number is = {}", v),
        Err(_) => println!("some thsing wrong happend")
    }
}
