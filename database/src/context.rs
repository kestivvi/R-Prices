use crate::db::PostgresPool;
use crate::models::offer::loader::OfferLoader;
use crate::models::price::loader::PriceLoader;

// The GraphQL context, which needs to provide everything necessary for
// interacting with the database.
pub struct GraphQLContext {
    pub pool: PostgresPool,
    pub offer_loader: OfferLoader,
    pub price_loader: PriceLoader,
    pub user_id: Option<i32>,
}

// This impl allows us to pass in GraphQLContext as the Context for GraphQL
// objects
impl juniper::Context for GraphQLContext {}
