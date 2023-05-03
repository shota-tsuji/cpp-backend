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
    async fn recipe(&self, ctx: &Context<'_>) -> Result<Recipe, String> {
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
}

#[derive(Debug, async_graphql::SimpleObject)]
pub struct Recipe {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(sqlx::FromRow)]
struct RecipeRow {
    id: i32,
    title: String,
    description: String,
}
