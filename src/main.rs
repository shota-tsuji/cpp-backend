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

async fn get_recipe_list(State(pool): State<MySqlPool>) -> Result<String, (StatusCode, String)> {
    let rec = sqlx::query!(r#"SELECT id, title, description FROM recipes"#)
        .fetch_one(&pool)
        .await
        .map_err(internal_error);

    let title = rec.unwrap().title;
    println!("{}", title);

    Ok(title)
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
