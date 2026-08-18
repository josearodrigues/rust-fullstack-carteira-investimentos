use crate::app::App;

mod app;
pub mod auth;
pub mod error;
pub mod handlers;
pub mod models;
pub mod repositories;
pub mod routes;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    App::start().await
}
