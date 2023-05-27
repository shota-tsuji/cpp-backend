use async_graphql::{Context, EmptyMutation, EmptySubscription, ID, Object, Schema};
use sqlx::mysql::MySqlPool;

use crate::presentation::graphql::mutation::Mutation;
use crate::presentation::graphql::object::Resource;

use super::object::{Recipe, RecipeDetail, Step};

pub type QuerySchema = Schema<Query, Mutation, EmptySubscription>;

pub struct Query {
    pool: MySqlPool,
}

impl Query {
    pub fn new(pool: MySqlPool) -> Self {
        Query { pool }
    }
}

#[Object]
impl Query {
    async fn recipe_detail(&self, _ctx: &Context<'_>, id: ID) -> Result<RecipeDetail, String> {
        let recipe_row: Option<RecipeRow> =
            sqlx::query_as(r#"SELECT id, title, description FROM recipes WHERE id = ?"#)
                .bind(id.as_str())
                .fetch_optional(&self.pool)
                .await
                .unwrap();

        let recipe = recipe_row
            .map(|row| Recipe {
                id: row.id,
                title: row.title,
                description: row.description,
            })
            .unwrap();

        let steps = sqlx::query_as("select id, description, resource_id, order_number, duration from steps where recipe_id = ?")
            .bind(id.as_str())
            .fetch_all(&self.pool)
            .await.unwrap().into_iter().map(|row: StepRow| {
            let id = row.id;
            let description = row.description;
            let resource_id = row.resource_id;
            let order_number = row.order_number;
            let duration = row.duration;
            Step {
                id,
                description,
                resource_id,
                order_number,
                duration,
            }
        }).collect();

        let recipe_detail = RecipeDetail {
            id: recipe.id,
            title: recipe.title,
            description: recipe.description,
            steps,
        };

        println!("{:?}", &recipe_detail);

        Ok(recipe_detail)
    }

    async fn recipes(&self, _ctx: &Context<'_>) -> Result<Vec<Recipe>, String> {
        let recipes = sqlx::query_as("SELECT * FROM recipes")
            .fetch_all(&self.pool)
            .await
            .unwrap()
            .into_iter()
            .map(|row: RecipeRow| {
                let id = row.id;
                let title = row.title;
                let description = row.description;
                println!("{:?}, {:?}, {:?}", id, title, description);
                Recipe {
                    id,
                    title,
                    description,
                }
            })
            .collect();

        Ok(recipes)
    }

    async fn process(&self, _ctx: &Context<'_>, id: ID) -> Result<Vec<RecipeDetail>, String> {
        println!("step0");
        let recipes: Vec<Recipe> = sqlx::query_as(r#"SELECT r.id, r.title, r.description FROM process_regsitrations AS pr LEFT JOIN recipes AS r ON pr.recipe_id = r.id WHERE pr.process_id = ?"#)
            .bind(id.as_str())
            .fetch_all(&self.pool)
            .await
            .unwrap()
            .into_iter()
            .map(|row: RecipeRow| {
                let id = row.id;
                let title = row.title;
                let description = row.description;
                println!("{:?}, {:?}, {:?}", id, title, description);
                Recipe {
                    id,
                    title,
                    description,
                }
            })
            .collect();

        println!("step1");
        let mut recipeDetails = Vec::new();
        for recipe in recipes {
            let steps = sqlx::query_as("select id, description, resource_id, order_number, duration from steps where recipe_id = ?")
                .bind(recipe.id.as_str())
                .fetch_all(&self.pool)
                .await.unwrap().into_iter().map(|row: StepRow| {
                let id = row.id;
                let description = row.description;
                let resource_id = row.resource_id;
                let order_number = row.order_number;
                let duration = row.duration;
                Step {
                    id,
                    description,
                    resource_id,
                    order_number,
                    duration,
                }
            }).collect();

            println!("step2");
            let recipe_detail = RecipeDetail {
                id: recipe.id,
                title: recipe.title,
                description: recipe.description,
                steps,
            };

            recipeDetails.push(recipe_detail);
        }

        println!("step3");
        Ok(recipeDetails)
    }

    async fn resource(&self, _ctx: &Context<'_>, id: ID) -> Result<Resource, String> {
        let resource_row: Option<ResourceRow> = sqlx::query_as(r#"SELECT id, name, amount FROM resources WHERE id = ?"#)
            .bind(id.as_str())
            .fetch_optional(&self.pool)
            .await
            .unwrap();

        let resource = resource_row.map(|row| Resource {
            id: row.id,
            name: row.name,
            amount: row.amount,
        }).unwrap();

        println!("{:?}", &resource);

        Ok(resource)
    }

    async fn resources(&self, _ctx: &Context<'_>) -> Result<Vec<Resource>, String> {
        let resources = sqlx::query_as("SELECT * FROM resources")
            .fetch_all(&self.pool)
            .await
            .unwrap()
            .into_iter()
            .map(|row: ResourceRow| {
                let id = row.id;
                let name = row.name;
                let amount = row.amount;
                println!("{:?}, {:?}, {:?}", id, name, amount);
                Resource {
                    id,
                    name,
                    amount,
                }
            }).collect();

        Ok(resources)
    }
}

#[derive(sqlx::FromRow)]
struct RecipeRow {
    id: String,
    title: String,
    description: String,
}

#[derive(sqlx::FromRow)]
struct StepRow {
    id: String,
    description: String,
    resource_id: u64,
    order_number: u32,
    duration: i32,
}

#[derive(sqlx::FromRow)]
struct ResourceRow {
    pub id: u64,
    pub name: String,
    pub amount: i32,
}
