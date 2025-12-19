use axum::{Json, Router, extract::State, routing::get};
use serde::Serialize;
use sqlx::PgPool;
use std::{env, net::SocketAddr, time::Duration};
use tower_http::cors::CorsLayer;

#[derive(Serialize, sqlx::FromRow)]
struct Message {
    id: i32,
    text: String,
}

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("DB接続に失敗しました");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("マイグレーションに失敗しました");

    // 1. ルート（パス）の設定
    let app = Router::new()
        .route("/api/hello", get(handler))
        // Next.js(3000番)からのアクセスを許可する設定（CORS）
        .layer(CorsLayer::permissive())
        .with_state(pool);

    // 2. サーバーの起動アドレス設定 (0.0.0.0:8000)
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("listening on {}", addr);

    // 3. サーバー起動
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler(State(pool): State<PgPool>) -> Json<Message> {
    tokio::time::sleep(Duration::from_secs(5)).await;
    let message = sqlx::query_as::<_, Message>("SELECT id, text FROM messages LIMIT 1")
        .fetch_one(&pool)
        .await
        .unwrap();
    Json(message)
}
