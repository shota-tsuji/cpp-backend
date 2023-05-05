use super::object::{Recipe, RecipeDetail, Step};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, ID};
use sqlx::mysql::MySqlPool;

pub type QuerySchema = Schema<Query, EmptyMutation, EmptySubscription>;

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
                    id, description, resource_id, order_number, duration,
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
    resource_id: i32,
    order_number: u32,
    duration: i32,
}
