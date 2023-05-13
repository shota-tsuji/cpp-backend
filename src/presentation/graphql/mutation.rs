use super::object::{CreateRecipeDetailInput, CreateStepInput, Recipe, RecipeDetail, Step};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, ID};
use sqlx::mysql::MySqlPool;
use uuid::Uuid;

pub struct Mutation {
    pool: MySqlPool,
}

impl Mutation {
    pub fn new(pool: MySqlPool) -> Self {
        Mutation { pool }
    }
}

#[Object]
impl Mutation {
    async fn create_recipe_detail(
        &self,
        _ctx: &Context<'_>,
        recipe_detail_data: CreateRecipeDetailInput,
    ) -> Result<RecipeDetail, String> {
        let recipe_id = Uuid::new_v4().to_string();
        println!("{}", recipe_id.clone());
        let query_result =
            sqlx::query(r#"INSERT INTO recipes (id, title, description) VALUES (?, ?, ?)"#)
                .bind(recipe_id.clone())
                .bind(recipe_detail_data.title.clone())
                .bind(recipe_detail_data.description.clone())
                .execute(&self.pool)
                .await
                .map_err(|err| err.to_string());
        if let Err(err) = query_result {
            eprintln!("{}", err);
        }

        let steps: Vec<Step> = recipe_detail_data
            .steps
            .into_iter()
            .map(|step| Step {
                id: Uuid::new_v4().to_string(),
                description: step.description,
                resource_id: step.resource_id,
                order_number: step.order_number,
                duration: step.duration,
            })
            .collect();
        for step in steps.iter() {
            println!("{}", recipe_id.clone());
            sqlx::query("INSERT INTO steps (id, recipe_id, description, resource_id, order_number, duration) VALUES (?, ?, ?, ?, ?, ?)")
                .bind(step.id.clone())
                .bind(recipe_id.clone())
                .bind(step.description.clone())
                .bind(step.resource_id)
                .bind(step.order_number)
                .bind(step.duration)
                .execute(&self.pool)
                .await
                .unwrap();
        }

        let recipe_detail = RecipeDetail {
            id: recipe_id.clone(),
            title: recipe_detail_data.title,
            description: recipe_detail_data.description,
            steps,
        };

        Ok(recipe_detail)
    }
}