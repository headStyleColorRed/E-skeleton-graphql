
use super::user::User;
use crate::schema::punches;

// The core data type undergirding the GraphQL interface

#[derive(Queryable)]
#[derive(juniper::GraphQLObject)]
pub struct Punch {
    pub id: i32,
    pub user_id: i32,
    pub entry: String,
    pub leave: String,
    pub created_at: String,
}