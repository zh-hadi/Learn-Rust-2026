use std::io;



fn main(){

   let mut input = String::new();
   
   io::stdin().read_line(&mut input).expect("read line error occors here");

   let data: Vec<i32>= input
                            .trim()
                            .split(",")
                            .map(|x| x.trim())
                            .filter_map(|x| x.parse::<i32>().ok())
                            .collect();

   println!("{:?}", data);
}