use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use cpp_backend::presentation::graphql::{mutation::Mutation, query::Query};
use sqlx::MySqlPool;
use std::env;

#[tokio::main]
async fn main() {
    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let query = Query::new(pool.clone());
    let mutation = Mutation::new(pool.clone());
    let schema = Schema::build(query, mutation, EmptySubscription).finish();
    print!("{}", schema.sdl());
}
