use diesel::prelude::*;
use diesel::PgConnection;
use juniper::FieldResult;

use super::Collection;
use super::CreateCollectionDiesel;
use crate::diesel_schema::collections;
use crate::models::utils;

pub fn create_collection(
    conn: &PgConnection,
    new_collection: CreateCollectionDiesel,
) -> FieldResult<Collection> {
    let res = diesel::insert_into(collections::table)
        .values(&new_collection)
        .get_result(conn);

    utils::graphql_translate(res)
}

pub fn delete(conn: &PgConnection, id: i32) -> FieldResult<Collection> {
    let res = diesel::delete(collections::table)
        .filter(collections::columns::id.eq(id))
        .get_result(conn);

    utils::graphql_translate(res)
}

// pub fn update_notification(
//     conn: &PgConnection,
//     product_id: i32,
//     user_id: i32,
//     new_value: bool,
// ) -> bool {
//     if new_value {
//         diesel::insert_into(diesel_schema::notifications::table)
//             .values(CreateNotificationRelation {
//                 product_id,
//                 user_id,
//             })
//             .execute(conn)
//             .is_ok()
//     } else {
//         diesel::delete(diesel_schema::notifications::table)
//             .filter(notifications::columns::product_id.eq(product_id))
//             .filter(notifications::columns::user_id.eq(user_id))
//             .execute(conn)
//             .is_ok()
//     }
// }

pub fn rename(
    conn: &PgConnection,
    collection_id: i32,
    new_name: String,
) -> FieldResult<Collection> {
    let res = diesel::update(collections::table)
        .filter(collections::columns::id.eq(collection_id))
        .set(collections::columns::name.eq(new_name))
        .get_result(conn);

    utils::graphql_translate(res)
}

pub fn change_description(
    conn: &PgConnection,
    collection_id: i32,
    new_description: String,
) -> FieldResult<Collection> {
    let res = diesel::update(collections::table)
        .filter(collections::columns::id.eq(collection_id))
        .set(collections::columns::description.eq(new_description))
        .get_result(conn);

    utils::graphql_translate(res)
}

pub fn change_visibility(
    conn: &PgConnection,
    collection_id: i32,
    public: bool,
) -> FieldResult<Collection> {
    let res = diesel::update(collections::table)
        .filter(collections::columns::id.eq(collection_id))
        .set(collections::columns::public.eq(public))
        .get_result(conn);

    utils::graphql_translate(res)
}
