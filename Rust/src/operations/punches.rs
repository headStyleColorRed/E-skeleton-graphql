
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
    pub fn all_punches(conn: &PgConnection) -> FieldResult<Vec<Punch>> {
        // Retrieve connection
        let res = punches.load::<Punch>(conn);
        // Return query
        graphql_translate(res)
    }
}

// Punch mutations
impl Punches {
    pub fn create_punch(conn: &PgConnection, input: CreateUserInput) -> FieldResult<Punch> {
        // Retrieve connection
        let res = diesel::insert_into(punches).values(&input).get_result(conn);
        graphql_translate(res)
    }
}

// The GraphQL input object for creating TODOs
#[derive(GraphQLInputObject, Insertable)]
#[table_name = "punches"]
pub struct CreateUserInput {
    pub user_id: i32,
    pub entry: String,
    pub leave: String,
}
