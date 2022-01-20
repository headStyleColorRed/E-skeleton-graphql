use super::models::Article;
use super::schema::articles;
use crate::schema::articles::dsl::*;
use diesel::{pg::PgConnection, Insertable, RunQueryDsl};
use juniper::{FieldResult, GraphQLInputObject};
use crate::utils::graphql_translate;

// This struct is basically a query manager. All the methods that it
// provides are static, making it a convenient abstraction for interacting
// with the database.
pub struct Articles;

// Article queries
impl Articles {
    pub fn all_articles(conn: &PgConnection) -> FieldResult<Vec<Article>> {
        // Retrieve connection
        let res = articles.load::<Article>(conn);
        // Return query
        graphql_translate(res)
    }
}

// Article mutations
impl Articles {
    pub fn create_article(conn: &PgConnection, input: CreateArticleInput) -> FieldResult<Article> {
        // Retrieve connection
        let res = diesel::insert_into(articles)
            .values(&input)
            .get_result(conn);
        graphql_translate(res)
    }
}

// The GraphQL input object for creating TODOs
#[derive(GraphQLInputObject, Insertable)]
#[table_name = "articles"]
pub struct CreateArticleInput {
    pub title: String,
    pub body: String,
}
