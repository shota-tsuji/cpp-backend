use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use axum::{extract::Extension, routing::get, Router};
use cpp_backend::presentation::{
    controller::graphql_controller::{graphiql, graphql_handler},
    graphql::{mutation::Mutation, query::Query},
};
use http::{
    header::{ACCEPT, CONTENT_TYPE},
    HeaderValue, Method,
};
use sqlx::mysql::MySqlPool;
use std::env;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() {
    let mut client = GreeterClient::connect("http://[::1]:50051").await.unwrap();

    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = MySqlPool::connect(&database_url).await.unwrap();

    let query = Query::new(pool.clone());
    let mutation = Mutation::new(pool.clone());
    let schema = Schema::build(query, mutation, EmptySubscription).finish();

    let origins = ["http://localhost:8000".parse().unwrap()];
    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::POST])
        .allow_headers(vec![ACCEPT, CONTENT_TYPE]);
    let cors_layer = ServiceBuilder::new().layer(cors);

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(cors_layer)
        .layer(Extension(schema));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
