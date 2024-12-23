use std::net::IpAddr;

pub mod database;
pub mod router;
pub mod state;

use database::connect::{connect_to_database, DB};
use eyre::Result;
use router::create_main_router;
use state::AppState;

pub struct App {
    address: IpAddr,
    port: u16,
    db: DB,
}

impl App {
    pub async fn new(port: u16, database_uri: &str) -> Result<Self> {
        let address = IpAddr::from([127, 0, 0, 1]);
        let db = connect_to_database(database_uri).await?;

        tracing_subscriber::fmt::init();
        Ok(Self { address, port, db })
    }

    pub async fn run(&self) -> Result<()> {
        let listener = tokio::net::TcpListener::bind((self.address, self.port)).await?;
        let state = AppState {
            db: self.db.clone(),
        };
        let router = create_main_router(state);

        tracing::info!("Server started at http://{}:{}", self.address, self.port);
        axum::serve(listener, router).await?;
        Ok(())
    }
}
