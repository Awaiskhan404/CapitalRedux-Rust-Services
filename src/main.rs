use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

mod db {
    pub mod db;
    pub mod schema;
}
mod routes {
    pub mod user_routes;
}


use db::db::{create_pool, PgPool};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = create_pool(&database_url).await.unwrap();

    let app_state = AppState { pool: pool.clone() };

    HttpServer::new(move || {
        App::new()
            .data(app_state.clone())
            .configure(routes::user_routes::user_routes)
            
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
