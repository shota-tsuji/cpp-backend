use async_graphql::{
    http::GraphiQLSource, Context, EmptyMutation, EmptySubscription, Object, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    async_trait,
    extract::{Extension, FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::{self, IntoResponse},
    routing::{get, post},
    Router,
};
use sqlx::mysql::MySqlPool;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let schema = Schema::build(
        Query { pool: pool.clone() },
        EmptyMutation,
        EmptySubscription,
    )
    .finish();

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub type QuerySchema = Schema<Query, EmptyMutation, EmptySubscription>;

async fn graphql_handler(schema: Extension<QuerySchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[derive(Debug, async_graphql::SimpleObject)]
pub struct Recipe {
    pub id: i32,
    pub title: String,
    pub description: String,
}

pub struct Query {
    pool: MySqlPool,
}

#[derive(sqlx::FromRow)]
struct RecipeRow {
    id: i32,
    title: String,
    description: String,
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
