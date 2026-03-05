use axum::{
   routing::get,
   Router,
   Json
};
use serde::{Deserialize, Serialize};
use serde_json::{Value,json};

#[derive(Serialize, Deserialize, Debug)]
struct User {
   id: i32,
   name: String,
   email: String,
}

#[tokio::main]
async fn main() {

   let app = Router::new().route("/", get(greeting))
                                 .route("/users", get(users));

   
   let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
   axum::serve(listener, app).await.unwrap();
}

async fn greeting()-> String
{
   "Hello world!".to_string()
}

async fn users()-> Json<Vec<User>>
{
   let user1 =  User{
      id: 1,
      name: "hadiuzzaman".to_string(),
      email: "zhhadi50@gmail.com".to_string()
   };
   let user: Vec<User> = vec![
      User {
         id: 1,
         name: "ab".to_string(),
         email: "ab@g.c".to_string()
      },
      User {
         id: 2,
         name: "hadi1c".to_string(),
         email: "hadi2@g.c".to_string()
      }
   ];

   let str = serde_json::to_string(&user1).unwrap();

   println!("{:?}", user1);
   println!("{}", str);
   // "all users".to_string()
   Json(user)
}