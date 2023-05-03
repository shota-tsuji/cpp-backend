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
use cpp_backend::presentation::{
    controller::graphql_controller::{graphiql, graphql_handler},
    graphql::query::Query,
};
use sqlx::mysql::MySqlPool;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let query = Query::new(pool.clone());
    let schema = Schema::build(query, EmptyMutation, EmptySubscription).finish();

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
