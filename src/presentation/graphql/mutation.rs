use async_graphql::{Context, EmptyMutation, EmptySubscription, ID, Object, Schema};
use sqlx::mysql::MySqlPool;
use uuid::Uuid;

use crate::presentation::graphql::object::{CreateResourceInput, Resource, UpdateResourceInput};

use super::object::{CreateRecipeDetailInput, CreateStepInput, Recipe, RecipeDetail, Step};

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

    async fn create_resource(&self, _ctx: &Context<'_>, resource_data: CreateResourceInput) -> Result<Resource, String> {
        let query_result =
            sqlx::query(r#"INSERT INTO resources (name, amount) VALUES (?, ?)"#)
                .bind(resource_data.name.clone())
                .bind(resource_data.amount)
                .execute(&self.pool)
                .await
                .unwrap();

        let resource = Resource {
            id: query_result.last_insert_id(),
            name: resource_data.name,
            amount: resource_data.amount,
        };

        Ok(resource)
    }

    async fn update_resource(&self, _ctx: &Context<'_>, resource_data: UpdateResourceInput) -> Result<Resource, String> {
        let query_result = sqlx::query(r#"UPDATE resources SET name=?, amount=? where id=?"#)
            .bind(resource_data.name.clone())
            .bind(resource_data.amount)
            .bind(resource_data.id)
            .execute(&self.pool)
            .await
            .unwrap();

        let resource = Resource {
            id: resource_data.id,
            name: resource_data.name,
            amount: resource_data.amount,
        };

        Ok(resource)
    }
}
