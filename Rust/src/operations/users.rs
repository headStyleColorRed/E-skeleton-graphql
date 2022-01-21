use crate::models::user::User;
use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::utils::graphql_translate;
use diesel::{pg::PgConnection, Insertable, RunQueryDsl};
use juniper::{FieldResult, GraphQLInputObject};

// This struct is basically a query manager. All the methods that it
// provides are static, making it a convenient abstraction for interacting
// with the database.
pub struct Users;

// User queries
impl Users {
    pub fn all_users(conn: &PgConnection) -> FieldResult<Vec<User>> {
        // Retrieve connection
        let res = users.load::<User>(conn);
        // Return query
        graphql_translate(res)
    }
}

// User mutations
impl Users {
    pub fn create_user(conn: &PgConnection, input: CreateUserInput) -> FieldResult<User> {
        // Retrieve connection
        let res = diesel::insert_into(users).values(&input).get_result(conn);
        graphql_translate(res)
    }
}

// The GraphQL input object for creating TODOs
#[derive(GraphQLInputObject, Insertable)]
#[table_name = "users"]
pub struct CreateUserInput {
    pub name: String,
    pub status: i32,
}
