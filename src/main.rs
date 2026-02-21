


fn main() {
    let ar = [20, 30, 3, 2, 40];
    let big_number = big_number_in_array(&ar);

    match big_number {
        Ok(v) => println!("big number is = {}", v),
        Err(e) => println!("{e}")
    }

    println!("{:?}", sort_array(&ar));
}




fn big_number_in_array(data: &[i32])->Result<i32, String>
{
    if data.is_empty(){
        return Err("Array is emtpy".to_string());
    }

    let mut big_number = data[0];

    for i in 0..data.len() {
        if data[i] > big_number {
            big_number = data[i];
        }
    }

    Ok(big_number)
}

fn sort_array(ar: &[i32])->Result<Vec<i32>, String>
{
    let mut vec = ar.to_vec();
    let len = vec.len();
    
    for i in 0..len {
        for j in 0..len - 1 - i {
            if vec[j] > vec[j+1] {
                let temp = vec[j];
                vec[j] = vec[j+1];
                vec[j+1] = temp;
            }
        }
    }

    Ok(vec)
}