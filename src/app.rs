use axum::Router;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{
    Layer, fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt,
};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub admin_token: String,
}

impl AppState {
    async fn new() -> color_eyre::Result<Self> {
        let admin_token = std::env::var("ADMIN_SECRET_KEY")?;
        let database_url = std::env::var("DATABASE_URL")?;
        let db = PgPool::connect(&database_url).await?;

        // Executa as migrações automaticamente
        sqlx::migrate!("./migrations")
            .run(&db)
            .await?;

        Ok(Self { db, admin_token })
    }
}

pub struct App;

impl App {
    pub async fn start() -> color_eyre::Result<()> {
        // Initialize tracing subscriber with a layer that logs new spans
        let layer = tracing_subscriber::fmt::layer()
            .with_span_events(FmtSpan::NEW)
            .boxed();

        tracing_subscriber::registry().with(layer).init();

        // Em desenvolvimento, carrega variáveis do arquivo .env.
        // Em produção/Docker, as variáveis já estão no ambiente — o .ok()
        // ignora graciosamente a ausência do arquivo sem encerrar o processo.
        dotenvy::dotenv().ok();
        let state = AppState::new().await?;

        let listener = TcpListener::bind("0.0.0.0:3000").await?;
        let router = Router::new()
            .nest("/api", crate::routes::api::router())
            .nest("/admin", crate::routes::admin::router())
            .merge(crate::routes::login::router())
            .merge(crate::routes::wallet::router())
            .merge(crate::routes::dashboard::router())
            .with_state(state);

        info!("Starting service");

        axum::serve(listener, router).await?;

        Ok(())
    }
}
