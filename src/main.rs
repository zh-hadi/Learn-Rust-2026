use std::collections::HashMap;

use axum::{
   Json, Router, extract::{ Path, Query, State}, http::{StatusCode, Uri}, response::IntoResponse, routing::get
};
use serde::{Deserialize, Serialize};
use serde_json::{Value,json};
use sqlx::{ MySqlPool};

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

   let db = "mysql://root:@localhost/todo_app";

   let pool = MySqlPool::connect(&db).await.expect("db connections failed!");

   let app = Router::new().route("/", get(greeting))
                                 .route("/users", get(users))
                                 .route("/str", get(str_test))
                                 .route("/data", get(res_test))
                                 .route("/path", get(path_test))
                                 .route("/query", get(path_query))
                                 .route("/todos", get(todos_list))
                                 .with_state(pool);

   
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


async fn path_test(Json(payload): Json<User>)-> impl IntoResponse
{
   (StatusCode::ACCEPTED, Json(payload))
}

async fn path_query(Query(params): Query<HashMap<String, String>>)-> impl IntoResponse
{
   println!("{:?}", params);

   (StatusCode::ACCEPTED, "query is ok".to_string())
}



async fn todos_list(
   State(pool): State<MySqlPool>
) -> impl IntoResponse {

   let todos = sqlx::query_as::<_, (i32, String)>(
       "SELECT id, data FROM todo"
   )
   .fetch_all(&pool)
   .await;

   match todos {
       Ok(data) => (StatusCode::OK, Json(data)).into_response(),
       Err(_) => (
           StatusCode::INTERNAL_SERVER_ERROR,
           "Database error"
       ).into_response(),
   }
}