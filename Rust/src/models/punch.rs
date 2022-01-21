#[derive(Queryable)]
#[derive(juniper::GraphQLObject)]
pub struct Punch {
    pub id: i32,
    pub user_id: i32,
    pub entry: String,
    pub leave: String,
    pub created_at: String,
}