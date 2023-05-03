use super::object::Recipe;
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
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
    async fn recipe(&self, _ctx: &Context<'_>) -> Result<Recipe, String> {
        let recipe_row: Option<RecipeRow> =
            sqlx::query_as(r#"SELECT id, title, description FROM recipes WHERE id = ?"#)
                .bind("0")
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

        println!("{:?}", &recipe);

        Ok(recipe)
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
    id: i32,
    title: String,
    description: String,
}
