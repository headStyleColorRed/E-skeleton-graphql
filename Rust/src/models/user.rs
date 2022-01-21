
use diesel::Queryable;

// The core data type undergirding the GraphQL interface
#[derive(Queryable)]
#[derive(juniper::GraphQLObject)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub status: i32,
}