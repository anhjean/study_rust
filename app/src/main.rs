mod api;
mod models;
mod repository;

use actix_web::{web::Data, web, App, HttpServer, HttpResponse, Responder, get, post};
use api::user_api::{create_user, delete_user, get_all_users, get_user, update_user};
use api::ipn_api::{create_ipn, get_ipn, update_ipn, delete_ipn};
use repository::mongodb_repo::MongoRepo;


// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .service(echo)
//             .route("/hey", web::get().to(manual_hello))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_user)
            .service(get_user)
            .service(update_user)
            .service(delete_user)
            .service(get_all_users)
            .service(hello)
            .service(echo)
            .service(create_ipn)
            .service(get_ipn)
            .service(update_ipn)
            .service(delete_ipn)

            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
