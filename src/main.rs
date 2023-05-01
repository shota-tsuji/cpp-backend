use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    routing::get,
    Router,
};
use sqlx::mysql::MySqlPool;
use std::env;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;
    let recs = sqlx::query!(r#"SELECT id, title, description FROM recipes"#)
        .fetch_all(&pool)
        .await?;

    for rec in recs {
        println!("{}, {}, {}", rec.id, rec.title, rec.description.unwrap(),);
    }

    Ok(())
}
