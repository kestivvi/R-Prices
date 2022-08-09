use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use juniper::{FieldError, FieldResult};

use crate::diesel_schema::prices;
use crate::models::price::Price;
use crate::models::utils;

pub fn all_prices(conn: &PgConnection) -> FieldResult<Vec<Price>> {
    let res = prices::table.load::<Price>(conn);
    utils::graphql_translate(res)
}

pub fn get_price_by_id(conn: &PgConnection, price_id: i32) -> FieldResult<Option<Price>> {
    let res = prices::table.find(price_id).get_result::<Price>(conn);
    match res {
        Ok(price) => Ok(Some(price)),
        Err(e) => match e {
            // Without this translation, GraphQL will return an error rather
            // than the more semantically sound JSON null if no price is found.
            diesel::result::Error::NotFound => FieldResult::Ok(None),
            _ => FieldResult::Err(FieldError::from(e)),
        },
    }
}

pub fn get_price_of_offer_id(conn: &PgConnection, offer_id: i32) -> FieldResult<Vec<Price>> {
    let res = prices::table
        .filter(prices::columns::offer_id.eq(offer_id))
        .get_results::<Price>(conn);
    utils::graphql_translate(res)
}

pub fn get_last_prices_of_offer(
    conn: &PgConnection,
    offer_id: i32,
    limit: i64,
) -> FieldResult<Vec<Price>> {
    // let conn = pool.get().unwrap();
    let res = prices::table
        .filter(prices::columns::offer_id.eq(offer_id))
        .order(prices::columns::created_at.desc())
        .limit(limit)
        .get_results(conn);
    utils::graphql_translate(res)
}
