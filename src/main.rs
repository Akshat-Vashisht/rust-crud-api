mod route;

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    Router,
};

use dotenv::dotenv;

pub mod api {
    pub mod category;
    pub mod items;
}

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_http::cors::CorsLayer;

#[derive(Clone, Debug)]
pub struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]

async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("Datbase URL must be provided");

    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database {:?}", err);
            std::process::exit(1);
        }
    };
    let state = AppState { db: pool };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = Router::new()
        .with_state(state.clone())
        .merge(route::config(state))
        .layer(cors);

    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
