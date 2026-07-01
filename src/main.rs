use axum::Router;
use briefly::{
    db::{pool::create_pool, shortner::ShortnerRepo, user::UserRepo},
    handlers::shortner::AppState,
    routes::{auth::auth_routes, shortner::shortner_routes},
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let pool = create_pool(&database_url).await?;
    sqlx::migrate!().run(&pool).await?;

    let state = AppState {
        shortner_repo: ShortnerRepo::new(pool.clone()),
        user_repo: UserRepo::new(pool),
        jwt_secret,
    };

    let app: Router = shortner_routes().merge(auth_routes()).with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("listening on port {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
