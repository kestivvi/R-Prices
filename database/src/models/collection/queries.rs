use crate::models::user::User;
use crate::models::utils;
use crate::{diesel_schema::*, models::product::Product};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use juniper::FieldResult;
use log::debug;

use super::Collection;

pub fn all_collections(conn: &PgConnection) -> FieldResult<Vec<Collection>> {
    let res = collections::table.load::<Collection>(conn);

    utils::graphql_translate(res)
}

pub fn all_public_collections(conn: &PgConnection) -> FieldResult<Vec<Collection>> {
    let res = collections::table
        .filter(collections::columns::public.eq(true))
        .get_results::<Collection>(conn);
    debug!("Public Collections: {:?}", res);
    utils::graphql_translate(res)
}

pub fn get_collection_by_id(conn: &PgConnection, id: i32) -> FieldResult<Collection> {
    let res = collections::table
        .filter(collections::columns::id.eq(id))
        .get_result::<Collection>(conn);
    utils::graphql_translate(res)
}

pub fn get_products_of_collection(
    conn: &PgConnection,
    collection_id: i32,
) -> FieldResult<Vec<Product>> {
    let res = collections::table
        .inner_join(collections_products_relation::table.inner_join(products::table))
        .filter(collections_products_relation::columns::collection_id.eq(collection_id))
        .select(products::all_columns)
        .load(conn);

    utils::graphql_translate(res)
}

pub fn get_owner_of_collection(conn: &PgConnection, collection_id: i32) -> FieldResult<User> {
    let res = users::table
        .inner_join(collections::table)
        .filter(collections::columns::id.eq(collection_id))
        .select(users::all_columns)
        .get_result(conn);

    utils::graphql_translate(res)
}

pub fn get_collections_of_user(conn: &PgConnection, user_id: i32) -> FieldResult<Vec<Collection>> {
    let res = users::table
        .inner_join(collections::table)
        .filter(users::columns::id.eq(user_id))
        .select(collections::all_columns)
        .get_results(conn);

    utils::graphql_translate(res)
}
