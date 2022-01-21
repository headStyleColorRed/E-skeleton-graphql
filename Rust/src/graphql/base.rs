use juniper::RootNode;

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
