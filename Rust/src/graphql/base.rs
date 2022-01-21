use juniper::RootNode;
use crate::context::GraphQLContext;
use crate::models::user::User;
use crate::operations::users::{CreateUserInput, Users};
use juniper::FieldResult;
use crate::models::punch::Punch;
use crate::operations::punches::{Punches, CreatePunchInput};

// The root GraphQL query
pub struct Query;

// The root GraphQL mutation
pub struct Mutation;

// And finally the root schema that pulls the query and mutation together. Perhaps someday
// you'll see a Subscription struct here as well.
pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}


// Queries
#[juniper::object(Context = GraphQLContext)]
impl Query {
    pub fn all_users(context: &GraphQLContext) -> FieldResult<Vec<User>> {
        Users::all_users(context)
    }

    pub fn all_punches(context: &GraphQLContext) -> FieldResult<Vec<Punch>> {
        Punches::all_punches(context)
    }
}

// Mutations
#[juniper::object(Context = GraphQLContext)]
impl Mutation {
    pub fn create_user(context: &GraphQLContext, input: CreateUserInput) -> FieldResult<User> {
        Users::create_user(context, input)
    }
    
    pub fn create_punch(context: &GraphQLContext, input: CreatePunchInput) -> FieldResult<Punch> {
        Punches::create_punch(context, input)
    }
}