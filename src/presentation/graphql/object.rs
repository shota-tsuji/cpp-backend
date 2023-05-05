#[derive(Debug, async_graphql::SimpleObject)]
pub struct Recipe {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Debug, async_graphql::SimpleObject)]
pub struct RecipeDetail {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub steps: Vec<Step>,
}

#[derive(Debug, async_graphql::SimpleObject)]
pub struct Step {
    pub id: i32,
    pub description: String,
    pub resource_id: i32,
    pub order_number: u32,
    pub duration: i32,
}
