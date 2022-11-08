use actix_web::{get, web, App, HttpServer, Responder};
use easy_json::Database;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: i32,
    username: String,
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    let database = Database::new(String::from("users"));
    let user = User {
        id: 123,
        username: String::from("test"),
    };
    database.add(user.clone());
    database.add(user.clone());
    database.remove(|x: &User| x.id == user.id);
    let vec = database.to_vec::<User>();
    println!("{vec:?}");
    format!("Hello {name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
