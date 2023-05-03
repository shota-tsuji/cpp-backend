#[derive(Debug, async_graphql::SimpleObject)]
pub struct Recipe {
    pub id: i32,
    pub title: String,
    pub description: String,
}
