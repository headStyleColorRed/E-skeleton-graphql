use super::context::GraphQLContext;
use diesel::pg::PgConnection;
use juniper::{FieldResult, RootNode};

use super::data::{Articles, CreateArticleInput};
use super::models::{Article};

// The root GraphQL query
pub struct Query;

// The root Query struct relies on GraphQLContext to provide the connection pool
// needed to execute actual Postgres queries.
#[juniper::object(Context = GraphQLContext)]
impl Query {
    pub fn all_articles(context: &GraphQLContext) -> FieldResult<Vec<Article>> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Articles::all_articles(conn)
    }
}

// The root GraphQL mutation
pub struct Mutation;

#[juniper::object(Context = GraphQLContext)]
impl Mutation {
    pub fn create_article(
        context: &GraphQLContext,
        input: CreateArticleInput,
    ) -> FieldResult<Article> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Articles::create_article(conn, input)
    }
}

// And finally the root schema that pulls the query and mutation together. Perhaps someday
// you'll see a Subscription struct here as well.
pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}
