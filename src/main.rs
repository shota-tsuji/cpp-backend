use async_graphql::{Context, Object};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    routing::get,
    Router,
};
use sqlx::mysql::MySqlPool;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let app = Router::new()
        .route("/", get(get_recipe_list))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_recipe_list(State(pool): State<MySqlPool>) -> Result<(), (StatusCode, String)> {
    let recipe_row: Option<RecipeRow> =
        sqlx::query_as("SELECT id, title, description FROM recipes WHERE id = ?")
            .bind("0")
            .fetch_optional(&pool)
            .await
            .unwrap();

    let recipe = recipe_row
        .map(|row| Recipe {
            id: row.id,
            title: row.title,
            description: row.description,
        })
        .unwrap();

    //let title = rec.unwrap().title;
    println!("{:?}", &recipe);

    Ok(())
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

#[derive(Debug, async_graphql::SimpleObject)]
pub struct Recipe {
    pub id: i32,
    pub title: String,
    pub description: String,
}

pub struct QueryRoot {
    pool: MySqlPool,
}

#[derive(sqlx::FromRow)]
struct RecipeRow {
    id: i32,
    title: String,
    description: String,
}

#[Object]
impl QueryRoot {
    async fn recipe(&self, ctx: &Context<'_>) -> Result<Recipe, String> {
        //let rec = sqlx::query!(r#"SELECT id, title, description FROM recipes"#)
        //    .fetch_one(&self.pool)
        //    .await
        //    .map_err(internal_error);
        let recipe_row: Option<RecipeRow> =
            sqlx::query_as(r#"SELECT id, title, description FROM recipes WHERE id = $1"#)
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

        //let title = rec.unwrap().title;
        println!("{:?}", &recipe);

        Ok(recipe)
    }
}
