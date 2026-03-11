use axum::{
   Json, Router, http::{StatusCode, Uri}, response::IntoResponse, routing::get
};
use serde::{Deserialize, Serialize};
use serde_json::{Value,json};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
   id: i32,
   name: String,
   email: String,
}

#[derive(Serialize)]
struct ApiResponse<T> {
   status: bool,
   data: T,
   message: String
}

#[tokio::main]
async fn main() {

   let app = Router::new().route("/", get(greeting))
                                 .route("/users", get(users))
                                 .route("/str", get(str_test))
                                 .route("/data", get(res_test));

   
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

async fn str_test()->String
{
   let st = "name".to_string();
   let st1 = " is hadiuzzaman".to_string();

   let st2 = st + &st1;
   let st3 = format!("{} {}", st1, st2);

   st3
}

async fn res_test(uri: Uri)->impl IntoResponse
{
   let name = "hadiuzzman hadi".to_string();
   let name = "shohag hossen".to_string();

   let number = vec![10, 20, 30, 40];

   let user = User {
      name: "hadiuzzaman".to_string(),
      email: "ab@gmail.com".to_string(),
      id: 25,
   };

   let users = vec![user.clone(), user.clone()];

   let api_response = ApiResponse {
      status: true,
      data: users,
      message: "all user view successfully!".to_string()
   };

   (StatusCode::NOT_FOUND, Json(api_response))
}