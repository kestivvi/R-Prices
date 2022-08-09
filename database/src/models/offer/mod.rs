pub mod loader;
pub mod mutations;
pub mod queries;

use juniper::FieldResult;

use crate::diesel_schema::{offers, products, products_offers_relation};
use crate::models::product::Product;
use crate::{context::GraphQLContext, models::price::Price};

#[derive(Queryable, Clone, Debug)]
pub struct Offer {
    pub id: i32,
    pub url: String,
}

#[juniper::graphql_object(context = GraphQLContext)]
impl Offer {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub async fn products(&self, context: &GraphQLContext) -> FieldResult<Vec<Product>> {
        let conn = context.pool.get()?;
        queries::get_products_of_offer(&conn, self.id)
    }

    pub async fn prices(&self, context: &GraphQLContext) -> Vec<Price> {
        context.price_loader.load(self.id).await
    }
}

#[derive(Insertable)]
#[table_name = "offers"]
pub struct CreateOfferInput {
    pub url: String,
}

#[derive(juniper::GraphQLInputObject)]
pub struct AddOfferInput {
    pub url: String,
    pub product_id: i32,
}

joinable!(products_offers_relation -> products (product_id));
joinable!(products_offers_relation -> offers (offer_id));
