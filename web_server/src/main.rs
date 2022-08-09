extern crate actix_rt;
extern crate actix_web;
extern crate dotenv;
extern crate env_logger;
extern crate juniper;

use actix_web::{middleware, web, App, HttpServer};
use env_logger::Env;
use std::{env, io};

use database::db::get_pool;
use web_server::{auth::endpoints::auth_endpoints, endpoints::graphql_endpoints};

fn init_env_and_logging() {
    dotenv::dotenv().ok();
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // Env variables from .env and logging setup
    init_env_and_logging();
    // TODO: Get db url from config file / config struct

    let url = env::var("DATABASE_URL").expect("no DB URL"); // TODO: handle errors
    let pool = get_pool(&url);

    // Start up the server, passing in:
    // (a) the connection pool to make it available to all endpoints
    // and (b) the configuration function that adds the /graphql logic.
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(actix_cors::Cors::permissive().allow_any_origin())
            .wrap(middleware::Logger::default())
            .configure(graphql_endpoints)
            .configure(auth_endpoints)
    })
    .workers(2)
    // TODO: Get addr from config file / config struct
    .bind("127.0.0.1:4000")?
    .run()
    .await
}
