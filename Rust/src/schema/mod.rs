mod health;

use async_graphql::{ MergedObject, SchemaBuilder, EmptyMutation, EmptySubscription, Schema };

#[derive(MergedObject, Default)]
pub struct Query(health::HealthQuery);

/// Build the Graphql schema
pub fn build_schema() -> SchemaBuilder<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
}