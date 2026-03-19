use std::collections::HashMap;

use axum::{
   Json, Router, extract::{ Path, Query, State}, http::{StatusCode, Uri}, response::IntoResponse, routing::{delete, get, post, put}
};
use::jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::{Value,json};
use sqlx::{ MySqlPool};
use chrono::{Utc, Duration};

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
   name: String,
   email: String,
   exp: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
   id: i32,
   name: String,
   email: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct Todo {
   id: i32,
   data: String
}

#[derive(Serialize)]
struct TodoResponse <T> {
   status: bool,
   message: String,
   data: T
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
                                 .route("/todos", get(todos_list).post(create_todo))
                                 .route("/todos/{id}", get(todos_get))
                                 .route("/todos/{id}", delete(todos_delete))
                                 .route("/todos/{id}/update", put(todos_update_handler))
                                 .route("/encode", get(encode_test))
                                 .route("/decode", get(decode_test))
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

   let todos = sqlx::query_as::<_, Todo>(
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

async fn todos_get(State(pool): State<MySqlPool>, Path(id): Path<i32>)->impl IntoResponse
{
   let query = "SELECT * FROM todo WHERE id = ?";
   let todo = sqlx::query_as::<_, Todo>(query).bind(id).fetch_one(&pool).await;

   match todo {
      Ok(data) => (StatusCode::OK, Json(data)).into_response(),
      Err(_) => (
         StatusCode::INTERNAL_SERVER_ERROR,
         "Database error"
      ).into_response(),
   }
}

async fn create_todo(
   State(pool): State<MySqlPool>,
   Json(payload): Json<Todo>
)->impl IntoResponse 
{
   let query = "INSERT INTO todo(data) VALUES (?)";

   let result = sqlx::query(query)
      .bind(payload.data)
      .execute(&pool)
      .await;

   match result {
      Ok(res) => {
         (StatusCode::CREATED, res.last_insert_id().to_string()).into_response()
      },
      Err(_) => (
         StatusCode::INTERNAL_SERVER_ERROR,
         "wrong data insert"
      ).into_response()
   }
}


async fn todos_delete(
   Path(id): Path<i32>,
   State(pool): State<MySqlPool>
)->impl IntoResponse
{
   let query = "DELETE FROM todo WHERE id = ?";

   let result = sqlx::query(query)
                     .bind(id)
                     .execute(&pool)
                     .await;

   match result {
      Ok(value) =>{
         if value.rows_affected() == 0 {

            let res = TodoResponse {
               status: false,
               message: format!("Todos this id {} not found", id),
               data: "todos not found".to_string()
            };

            (StatusCode::NOT_FOUND, Json(res))
         }else {
            let res = TodoResponse {
               status: true,
               message: format!("Todos deleted id {}", id),
               data: "".to_string()
            };

            (StatusCode::OK, Json(res))
         }
      },
      Err(_) => {
         let res = TodoResponse {
            status: false,
            message: format!("Todos this id {} not found", id),
            data: "something went wrong to delete data".to_string()
         };

         (StatusCode::SERVICE_UNAVAILABLE, Json(res))
         
      }
         
   }
}


async fn todos_update_handler(
   State(pool): State<MySqlPool>,
   Json(payload): Json<Todo>
)-> impl IntoResponse
{
   let query = "UPDATE todo SET data = ? WHERE id = ?";

   let result = sqlx::query(query)
                        .bind(&payload.data)
                        .bind(payload.id)
                        .execute(&pool)
                        .await;
   
   match result {
      Ok(value)=> {
         if value.rows_affected() == 0 {
            let res = TodoResponse{
               status: false,
               message: format!("Todo with id {} not found", payload.id),
               data: "".to_string()
            };

            (StatusCode::NOT_FOUND, Json(res))
         }else {
            let res = TodoResponse {
               status: true,
               message: format!("Todo with id {} update Successfully!", payload.id),
               data: payload.data,
            };

            (StatusCode::OK, Json(res))
         }
      },
      Err(err) => {
         eprintln!("Update error: {:?}", err);

         let res = TodoResponse {
            status: false,
            message: "Failed to update todo!".to_string(),
            data: "".to_string()
         };

         (StatusCode::INTERNAL_SERVER_ERROR, Json(res))
      }
   }
}



async fn encode_test()-> impl IntoResponse
{
   let claims_data = Claims{
      name: "hadiuzzaman".to_string(),
      email: "zhhadi50@gmail.com".to_string(),
      exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
   };

   let token = encode(&Header::default(), &claims_data, &EncodingKey::from_secret("hadi".as_ref()));

   match token {
      Ok(value) => {
         println!("TOKEN: {}", value);
         (StatusCode::OK, value).into_response()
      },
      Err(_) => {
         (StatusCode::INTERNAL_SERVER_ERROR, "something wrong happend".to_string()).into_response()
      }
   }
}

async fn decode_test()->Result<impl IntoResponse, StatusCode>
{
   let code = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJuYW1lIjoiaGFkaXV6emFtYW4iLCJlbWFpbCI6InpoaGFkaTUwQGdtYWlsLmNvbSIsImV4cCI6MTc3MzgyMTgyNX0.yDcLLp1rqyaWh_ts2lmvOTEX4PzsyIt2RI5rDV5jl60";

   let mut validation = Validation::default();
   validation.validate_exp = false;

   // let decode = decode::<Claims>(&code, &DecodingKey::from_secret("hadi".as_ref()), &validation).map_err(|_| StatusCode::UNAUTHORIZED)?;
   match decode::<Claims>(
      &code,
      &DecodingKey::from_secret("hadi".as_ref()),
      &validation
   ) {
      Ok(data) => {
         println!("SUCCESS: {:?}", data);
         Ok((StatusCode::OK, "ok"))
      },
      Err(e) => {
         println!("ERROR: {:?}", e); 
         Err(StatusCode::UNAUTHORIZED)
      }
   }

   // Ok((StatusCode::OK, "decode test".to_string()))
}