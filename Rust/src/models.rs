
use diesel::Queryable;

// The core data type undergirding the GraphQL interface
#[derive(Queryable)]
#[derive(juniper::GraphQLObject)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}