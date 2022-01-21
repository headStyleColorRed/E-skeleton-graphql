use crate::context::GraphQLContext;
use crate::models::punch::Punch;
use crate::operations::punches::{CreateUserInput, Punches};
use diesel::pg::PgConnection;
use juniper::FieldResult;

use super::base::{Mutation, Query};

// The root Query struct relies on GraphQLContext to provide the connection pool
// needed to execute actual Postgres queries.
impl Query {
    pub fn all_punches(context: &GraphQLContext) -> FieldResult<Vec<Punch>> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Punches::all_punches(conn)
    }
}

impl Mutation {
    pub fn create_punch(context: &GraphQLContext, input: CreateUserInput) -> FieldResult<Punch> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Punches::create_punch(conn, input)
    }
}
