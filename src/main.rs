
struct  User {
   name: String,
   age: i32,
}

impl User {
   fn new(name: String, age: i32)-> User {
      User {
         name: name,
         age: age
      }
   }

   fn veiw_user_info(&self)
   {
      println!("user name: {}", self.name);
      println!("user age: {}", self.age);
   }

   fn dummy(){
      println!("dummy method this is ");
   }
}

fn main(){

   let user1 = User::new("Hadiuzzaman".to_string(), 25);
   user1.veiw_user_info();
   
   User::dummy();
   
}