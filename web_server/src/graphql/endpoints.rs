use actix_web::{web, Error, HttpResponse};
use actix_web::{HttpRequest, Responder};
use juniper::http::playground::playground_source;
use juniper::http::GraphQLRequest;
use log::debug;
use std::sync::Arc;

use crate::graphql::schema::{create_schema, Schema};
use database::context::GraphQLContext;
use database::db::PostgresPool;

use database::models;

// The configuration callback that enables us to add the /graphql route
// to the actix-web server.
pub fn graphql_endpoints(config: &mut web::ServiceConfig) {
    let schema = Arc::new(create_schema());
    config
        .app_data(web::Data::new(schema))
        .route("/graphql", web::post().to(graphql))
        .route("/graphql", web::get().to(graphql_playground));
}

// The GraphQL Playground route.
async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source("/graphql", None))
}

// The core handler that provides all GraphQL functionality.
async fn graphql(
    request: HttpRequest,
    // The DB connection pool
    pool: web::Data<PostgresPool>,
    // The GraphQL schema
    schema: web::Data<Arc<Schema>>,
    // The incoming HTTP request
    data: web::Json<GraphQLRequest>,
) -> actix_web::Result<impl Responder> {
    debug!("Request Cookies: {:?}", request.cookies());
    let user_id = request
        .cookie("session_id")
        .and_then(|v| v.value().parse::<i32>().ok())
        .and_then(|session_id| {
            database::models::session::queries::get_user_id(&pool.get().unwrap(), session_id)
        });

    // Instantiate a context
    let ctx = GraphQLContext {
        pool: pool.get_ref().to_owned(),
        offer_loader: models::offer::loader::get_offer_loader(pool.get_ref().to_owned()),
        price_loader: models::price::loader::get_price_loader(pool.get_ref().to_owned()),
        user_id,
    };
    debug!("State of the pool {:?}", ctx.pool.state());
    // debug!("{}", )
    // Handle the incoming request and return a string result (or error)
    let res = data.execute(&schema, &ctx);
    let res = serde_json::to_string(&res.await).map_err(Error::from)?;

    // Return the string as a JSON payload
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(res))
}
