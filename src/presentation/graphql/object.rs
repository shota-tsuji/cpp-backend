use async_graphql::{InputObject, SimpleObject};

#[derive(Debug, SimpleObject)]
pub struct Recipe {
    pub id: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, SimpleObject)]
pub struct RecipeDetail {
    pub id: String,
    pub title: String,
    pub description: String,
    pub steps: Vec<Step>,
}

#[derive(Debug, SimpleObject)]
pub struct Resource {
    pub id: i32,
    pub name: String,
    pub amount: i32,
}

impl RecipeDetail {
    pub fn new(id: String, title: String, description: String, steps: Vec<Step>) -> Self {
        Self {
            id,
            title,
            description,
            steps,
        }
    }
}

#[derive(Debug, SimpleObject)]
pub struct Step {
    pub id: String,
    pub description: String,
    pub resource_id: i32,
    pub order_number: u32,
    pub duration: i32,
}

#[derive(InputObject)]
pub struct CreateRecipeDetailInput {
    pub title: String,
    pub description: String,
    pub steps: Vec<CreateStepInput>,
}

#[derive(InputObject)]
pub struct CreateStepInput {
    pub description: String,
    pub resource_id: i32,
    pub order_number: u32,
    pub duration: i32,
}
