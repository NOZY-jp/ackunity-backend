use axum::{Json, Router, routing::get};
use serde::Serialize;
use sqlx::PgPool;
use std::{env, net::SocketAddr};
use tower_http::cors::CorsLayer;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum Severity {
    Critical,
    Important,
    Info,
}

#[derive(Serialize)]
struct Notification {
    id: String,
    org_id: String,
    source: String,
    severity: Severity,
    title: String,
    content: String,
    created_at: String,
    ack_count: Option<i32>,
    view_count: Option<i32>,
    github_hash: Option<String>,
}

#[derive(Serialize)]
struct Organization {
    id: String,
    name: String,
}

async fn get_notifications() -> Json<Vec<Notification>> {
    let notifications = vec![
        Notification {
            id: "1".into(),
            org_id: "550e8400-e29b-41d4-a716-446655440000".into(),
            source: "MANUAL".into(),
            severity: Severity::Critical,
            title: "【重要】ハッカソン最終プレゼンについて".into(),
            content: "プレゼン資料の提出期限は本日23:59です。デモ動画のリンクも忘れずに設定してください。未提出の場合は審査対象外となります。".into(),
            created_at: "2025-12-19 12:00".into(),
            ack_count: Some(12),
            view_count: None,
            github_hash: None,
        },
        Notification {
            id: "2".into(),
            org_id: "ad4fbc32-1234-5678-90ab-cdef12345678".into(),
            source: "GITHUB".into(),
            severity: Severity::Important,
            title: "New PR: feat/notification-pool".into(),
            content: "frontendコンポーネントの実装が完了しました。レビュアーによる承認が必要です。".into(),
            created_at: "2025-12-19 11:30".into(),
            ack_count: Some(5),
            view_count: None,
            github_hash: Some("#82ef12b".into()),
        },
        Notification {
            id: "3".into(),
            org_id: "bcdef012-3456-789a-bcde-f0123456789a".into(),
            source: "SYSTEM".into(),
            severity: Severity::Info,
            title: "今週のコンテスト案内".into(),
            content: "明日の21時からABC（AtCoder Beginner Contest）が開催されます。".into(),
            created_at: "2025-12-19 09:00".into(),
            ack_count: None,
            view_count: Some(245),
            github_hash: None,
        },
    ];
    Json(notifications)
}

async fn get_organizations() -> Json<Vec<Organization>> {
    // Rust側でダミーデータを返す
    let orgs = vec![
        Organization {
            id: "550e8400-e29b-41d4-a716-446655440000".to_string(),
            name: "Ritsumeikan Univ.".to_string(),
        },
        Organization {
            id: "678f9012-bcd3-4567-8901-234567890123".to_string(),
            name: "Ackunity Team".to_string(),
        },
        Organization {
            id: "bcdef012-3456-789a-bcde-f0123456789a".to_string(),
            name: "Competitive Programming".to_string(),
        },
    ];
    Json(orgs)
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
        .route("/api/organizations", get(get_organizations))
        .route("/api/notifications", get(get_notifications))
        .layer(CorsLayer::permissive())
        .with_state(pool);

    // 2. サーバーの起動アドレス設定 (0.0.0.0:8000)
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("listening on {}", addr);

    // 3. サーバー起動
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
