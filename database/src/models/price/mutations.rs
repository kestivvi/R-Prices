use diesel::{PgConnection, RunQueryDsl};
use juniper::FieldResult;

use crate::diesel_schema::prices;
use crate::models::price::{CreatePriceInput, Price};
use crate::models::utils;

pub fn create_price(conn: &PgConnection, new_price: &CreatePriceInput) -> FieldResult<Price> {
    let res = diesel::insert_into(prices::table)
        .values(new_price)
        .get_result(conn);

    utils::graphql_translate(res)
}
