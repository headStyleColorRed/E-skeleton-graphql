use crate::context::GraphQLContext;
use crate::models::user::User;
use crate::operations::users::{CreateUserInput, Users};
use diesel::pg::PgConnection;
use juniper::FieldResult;

use super::base::{Mutation, Query};

// The root Query struct relies on GraphQLContext to provide the connection pool
// needed to execute actual Postgres queries.
#[juniper::object(Context = GraphQLContext)]
impl Query {
    pub fn all_users(context: &GraphQLContext) -> FieldResult<Vec<User>> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Users::all_users(conn)
    }
}

#[juniper::object(Context = GraphQLContext)]
impl Mutation {
    pub fn create_user(context: &GraphQLContext, input: CreateUserInput) -> FieldResult<User> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Users::create_user(conn, input)
    }
}
