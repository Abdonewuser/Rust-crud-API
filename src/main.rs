use std::{collections::HashMap, sync::Mutex};

use actix_web::{web::{self, get}, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;


struct User {
    id: u64,
    name: String
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
}

struct AppState {
    state: Mutex<HashMap<u64, User>>
}

async fn get_name() -> impl Responder {
    "hello"
}

async fn set_name(input: web::Json<CreateUser>, data: web::Data<AppState>) -> impl Responder {
    let mut list = data.state.lock().unwrap();
    let new_id = list.len() as u64 + 1;

    let new_user = User {
        id: new_id,
        // without clone() This will cause a compiler error because input is an immutable reference, and you can't move a value out of it.
        name: input.name.clone()
    };
    list.insert(new_id, new_user);

    HttpResponse::Ok().body(input.name.clone())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState{
        state: Mutex::new(HashMap::new())
    });
    HttpServer::new(move || {
        App::new()
        .app_data(app_state.clone())
        .route("/user",web::get().to(get_name))
        .route("/user", web::post().to(set_name))
    })
        
        .bind("127.0.0.1:8080")?
        .run()
        .await
}