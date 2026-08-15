use crate::app::App;

mod app;
pub mod auth;
pub mod models;
pub mod routes;
pub mod error;
pub mod repository;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    App::start().await
}
