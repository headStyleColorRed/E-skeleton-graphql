// Global crates
#[macro_use]
extern crate diesel;

use actix_web::{middleware, App, HttpServer};
use std::{env, io};

mod utils;
mod schema;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    logging_setup();

    // Instantiate a new connection pool
    let pool = utils::db::get_pool();

    // Start up the server, passing in (a) the connection pool
    // to make it available to all endpoints and (b) the configuration
    // function that adds the /graphql logic.
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(utils::endpoints::graphql_endpoints)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// TODO: more fine-grained logging setup
fn logging_setup() {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
}
