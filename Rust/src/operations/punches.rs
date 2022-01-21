use crate::context::GraphQLContext;
use crate::models::punch::Punch;
use crate::schema::punches;
use crate::schema::punches::dsl::*;
use crate::utils::graphql_translate;
use diesel::{pg::PgConnection, Insertable, RunQueryDsl};
use juniper::{FieldResult, GraphQLInputObject};

// This struct is basically a query manager. All the methods that it
// provides are static, making it a convenient abstraction for interacting
// with the database.
pub struct Punches;

// Punch queries
impl Punches {
    pub fn all_punches(context: &GraphQLContext) -> FieldResult<Vec<Punch>> {
        // Retrieve connection
        let conn: &PgConnection = &context.pool.get().unwrap();
        // Make query
        let res = punches.load::<Punch>(conn);
        // Parse result
        graphql_translate(res)
    }
}

// Punch mutations
impl Punches {
    pub fn create_punch(context: &GraphQLContext, input: CreatePunchInput) -> FieldResult<Punch> {
        // Retrieve connection
        let conn: &PgConnection = &context.pool.get().unwrap();
        // Make query
        let res = diesel::insert_into(punches).values(&input).get_result(conn);
        // Parse result
        graphql_translate(res)
    }
}

// The GraphQL input object for creating TODOs
#[derive(GraphQLInputObject, Insertable)]
#[table_name = "punches"]
pub struct CreatePunchInput {
    pub user_id: i32,
    pub entry: String,
    pub leave: String,
}
