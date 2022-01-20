// Global crates
#[macro_use]
extern crate diesel;

use actix_web::{middleware, App, HttpServer};
use std::{env, io};
mod db;
mod data;
mod context;
mod endpoints;
mod graphql;
mod models;
mod schema;
mod utils;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // Logging setup
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Instantiate a new connection pool
    let pool = db::get_pool();

    // Start up the server, passing in (a) the connection pool
    // to make it available to all endpoints, (b) the logger middelware to 
    // know that is happening and (b) the configuration function that adds the /graphql logic.
    // TODO: - Add cors
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(endpoints::graphql_endpoints)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}