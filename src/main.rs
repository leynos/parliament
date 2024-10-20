mod entity;
mod user;

use crate::entity::Entity;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::{io, sync::Arc};
use user::{User, UserBody};

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/v1/user/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[post("/v1/user/")]
async fn hello_post(
    user_body: web::Form<UserBody>,
    db: web::Data<Arc<indradb_proto::Client>>,
) -> impl Responder {
    let user = User::new(user_body.name.to_string());

    // Attempt to get a mutable reference to the inner Client
    match Arc::try_unwrap(db.as_ref().clone()) {
        Ok(mut db_mut) => {
            // Call the save method with the mutable reference
            if let Err(err) = user.save(&mut db_mut).await {
                // Handle the error appropriately
                return HttpResponse::InternalServerError()
                    .body(format!("Error saving user: {}", err));
            }
            HttpResponse::Ok().body("User saved successfully")
        }
        _ => HttpResponse::InternalServerError().body("Database client is currently in use."),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let indra_client = indradb_proto::Client::new(
        "grpc://127.0.0.1:27615"
            .try_into()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?,
    )
    .await
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(indra_client.clone()))
            .service(index)
            .service(hello)
            .service(hello_post)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
