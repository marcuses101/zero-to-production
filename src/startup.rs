use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgConnection;
use std::net::TcpListener;

use crate::routes::{greet, health_check, subscribe};

pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);
    let port = listener.local_addr().unwrap().port();
    println!("listening at http://127.0.0.1:{}", port);
    let server: Server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/{name}", web::get().to(greet))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
