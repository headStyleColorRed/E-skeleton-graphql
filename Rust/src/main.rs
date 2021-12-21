#[macro_use]
extern crate juniper;

use std::io;
use std::sync::Arc;

use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};
use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
mod schema;
use schema::graphql_schema;
use schema::graphql_schema::Schema;

fn main() -> io::Result<()> {
    println!("Running server on port 8080");
    let schema = Arc::new(graphql_schema::create_schema());

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .route("/health", web::get().to(health))
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/playground").route(web::get().to(playground)))
    })
    .bind("localhost:8080")?
    .run()
}

fn health() -> impl Responder {
    HttpResponse::Ok().body("Server is up and running")
}

fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(user))
    })
}

fn playground() -> HttpResponse {
    let html = graphiql_source("http://localhost:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}