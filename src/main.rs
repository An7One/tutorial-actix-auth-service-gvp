#[macro_use]
extern crate validator_derive;

mod config;
mod db;
mod error;
mod handler;
mod model;

use actix_web::{App,HttpServer,middleware::Logger};
use color_eyre::Result;
use tracing::{info, instrument};

use crate::config::Config;
use crate::handler::app_config;

#[actix_rt::main]
#[instrument]
async fn main() -> Result<()>{

    let config = Config::from_env()
        .expect("Server configuration");

    let pool = config.db_pool().await.expect("Database configuration");
    
    let hashing = config.hashing(); 

    info!("Staring the server at http://{}:{}", config.host, config.port);

    HttpServer::new(move ||{
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .data(hashing.clone())
            .configure(app_config)
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;

    Ok(())
}
