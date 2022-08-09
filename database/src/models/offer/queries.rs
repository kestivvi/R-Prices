use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use juniper::FieldResult;

use crate::diesel_schema::{offers, products, products_offers_relation};
use crate::models::offer::Offer;
use crate::models::product::Product;
use crate::models::utils;

pub fn all_offers(conn: &PgConnection) -> FieldResult<Vec<Offer>> {
    let res = offers::table.load::<Offer>(conn);
    utils::graphql_translate(res)
}

pub fn offers_of_product(conn: &PgConnection, product_id: i32) -> FieldResult<Vec<Offer>> {
    let res = offers::table
        .inner_join(products_offers_relation::table.inner_join(products::table))
        .filter(products_offers_relation::columns::product_id.eq(product_id))
        .select(offers::all_columns)
        .get_results(conn);
    utils::graphql_translate(res)
}

pub fn offer_by_id(conn: &PgConnection, offer_id: i32) -> FieldResult<Offer> {
    let res = offers::table.find(offer_id).get_result::<Offer>(conn);
    utils::graphql_translate(res)
}

pub fn get_products_of_offer(conn: &PgConnection, offer_id: i32) -> FieldResult<Vec<Product>> {
    let res = offers::table
        .inner_join(products_offers_relation::table.inner_join(products::table))
        .filter(products_offers_relation::columns::offer_id.eq(offer_id))
        .select(products::all_columns)
        .get_results(conn);
    utils::graphql_translate(res)
}

pub fn get_offer_by_url(conn: &PgConnection, url: &str) -> Option<Offer> {
    offers::table
        .filter(offers::columns::url.eq(url))
        .get_result::<Offer>(conn)
        .ok()
}
