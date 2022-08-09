use diesel::ExpressionMethods;
use diesel::PgConnection;
use diesel::RunQueryDsl;
use juniper::FieldResult;

use crate::diesel_schema::offers;
use crate::models::offer::Offer;
use crate::models::utils;

// pub fn create_offer(conn: &PgConnection, new_offer: CreateOfferInput) -> FieldResult<Offer> {
//     let res = diesel::insert_into(offers::table)
//         .values(&new_offer)
//         .get_result(conn);

//     utils::graphql_translate(res)
// }

pub fn change_url(conn: &PgConnection, id: i32, new_value: String) -> FieldResult<Offer> {
    let res = diesel::update(offers::table)
        .filter(offers::columns::id.eq(id))
        .set(offers::columns::url.eq(new_value))
        .get_result(conn);

    utils::graphql_translate(res)
}

pub fn delete_offer(conn: &PgConnection, id: i32) -> FieldResult<Offer> {
    let res = diesel::delete(offers::table)
        .filter(offers::columns::id.eq(id))
        .get_result(conn);

    utils::graphql_translate(res)
}
