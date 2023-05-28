use async_graphql::{InputObject, SimpleObject};

#[derive(Debug, SimpleObject)]
pub struct Recipe {
    pub id: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, SimpleObject, InputObject)]
// Also used as input for UpdateRecipeDetail mutation
// update whole recipe detail (recipe and related steps) at once
#[graphql(input_name = "UpdateRecipeDetailInput")]
pub struct RecipeDetail {
    pub id: String,
    pub title: String,
    pub description: String,
    pub steps: Vec<Step>,
}

#[derive(Debug, SimpleObject)]
pub struct Resource {
    pub id: u64,
    pub name: String,
    pub amount: i32,
}

#[derive(Debug, SimpleObject)]
pub struct HelloResponse {
    pub message: String,
}

#[derive(Debug, SimpleObject)]
pub struct Process {
    pub resource_infos: Vec<ResourceInfo>,
    //pub step_results: Vec<StepResult>,
}

#[derive(Debug, SimpleObject)]
pub struct ResourceInfo {
    pub id: u64,
    pub name: String,
    pub steps: Vec<StepResult>,
    //pub is_used_multiple_resources: bool,
    //pub used_resources_count: i32,
}

#[derive(Debug, SimpleObject, Clone)]
pub struct StepResult {
    pub id: String,
    pub recipe_id: String,
    pub resource_id: u64,
    pub start_time: i32,
    pub duration: i32,
    pub order_number: u32,
    pub timeline_index: i32,
    pub description: String,
    pub recipe_name: String,
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

#[derive(Debug, SimpleObject, InputObject)]
// Different name is needed to avoid runtime error about GraphQL objects.
#[graphql(input_name = "StepInput")]
pub struct Step {
    pub id: String,
    pub description: String,
    pub resource_id: u64,
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
    pub resource_id: u64,
    pub order_number: u32,
    pub duration: i32,
}

#[derive(InputObject)]
pub struct CreateResourceInput {
    pub name: String,
    pub amount: i32,
}

#[derive(InputObject)]
pub struct UpdateResourceInput {
    pub id: u64,
    pub name: String,
    pub amount: i32,
}
