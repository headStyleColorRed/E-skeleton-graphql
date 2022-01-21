use super::punch::Punch;
use crate::{context::GraphQLContext, utils::graphql_translate};
use diesel::{Queryable, PgConnection, RunQueryDsl};
use juniper::{FieldResult};
use crate::schema::users::dsl::*;

use crate::operations::punches::{Punches};
use crate::schema::punches::dsl::*;

// The core data type undergirding the GraphQL interface
#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub status: i32,
}

#[juniper::object(Context = GraphQLContext)]
impl User {
    fn id(&self) -> i32 {
        self.id
    }
    fn name(&self) -> &str {
        self.name.as_str()
    }
    fn status(&self) -> i32 {
        self.status
    }

    pub fn all_users(context: &GraphQLContext) -> FieldResult<Vec<Punch>> {
        let conn: &PgConnection = &context.pool.get().unwrap();
        // Retrieve connection
        let res = punches.load::<Punch>(conn);
        // Return query
        graphql_translate(res)
    }
}
