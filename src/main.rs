#[macro_use]
extern crate validator_derive;

mod config;
mod handler;

use actix_web::{App,HttpServer,middleware::Logger};
use color_eyre::Result;
use crate::config::Config;
use crate::handler::app_config;
use tracing::info;

#[actix_rt::main]
async fn main() -> Result<()>{

    let config = Config::from_env()
        .expect("Server configuration");

    let pool = config.db_pool().await.expect("Database configuration");
    
    info!("Staring the server at http://{}:{}", config.host, config.port);

    HttpServer::new(move ||{
        App::new()
        .wrap(Logger::default())
        .data(pool.clone())
        .configure(app_config)
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;

    Ok(())
}
