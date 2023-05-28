use async_graphql::{Context, EmptyMutation, EmptySubscription, ID, Object, Schema};
use sqlx::mysql::MySqlPool;
use uuid::Uuid;

use crate::presentation::graphql::object::{CreateProcessInput, CreateResourceInput, ProcessId, Resource, UpdateResourceInput};

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

    async fn update_recipe_detail(
        &self,
        _ctx: &Context<'_>,
        recipe_detail_data: RecipeDetail,
    ) -> Result<RecipeDetail, String> {
        println!("received: {:?}", recipe_detail_data);
        let query_result = sqlx::query(r#"UPDATE recipes SET title=?, description=? where id=?"#)
            .bind(recipe_detail_data.title.clone())
            .bind(recipe_detail_data.description.clone())
            .bind(recipe_detail_data.id.clone())
            .execute(&self.pool)
            .await
            .unwrap();
        for step in recipe_detail_data.steps.iter() {
            println!("{:?}", step.clone());
            sqlx::query("UPDATE steps SET description=?, resource_id=?, order_number=?, duration=? where id=?")
                .bind(step.description.clone())
                .bind(step.resource_id)
                .bind(step.order_number)
                .bind(step.duration)
                .bind(step.id.clone())
                .execute(&self.pool)
                .await
                .unwrap();
        }

        Ok(recipe_detail_data)
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

    async fn create_process(&self, _ctx: &Context<'_>, recipe_id_list: CreateProcessInput) -> Result<ProcessId, String> {
        let query_result =
            sqlx::query(r#"INSERT INTO processes (name) VALUES (?)"#)
                .bind("process")
                .execute(&self.pool)
                .await
                .unwrap();
        let process_id = query_result.last_insert_id();

        for recipe_id in recipe_id_list.recipe_id_list {
            sqlx::query("INSERT INTO process_regsitrations (process_id, recipe_id) VALUES (?, ?)")
                .bind(process_id)
                .bind(recipe_id.clone())
                .execute(&self.pool)
                .await
                .unwrap();
        }

        Ok(ProcessId {
            id: process_id
        })
    }
}
