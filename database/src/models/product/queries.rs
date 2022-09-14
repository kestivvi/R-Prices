use crate::models::collection::Collection;
use crate::models::product::Product;
use crate::models::utils;
use crate::{diesel_schema::*, models::user::User};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use juniper::FieldResult;

pub fn all_products(conn: &PgConnection) -> FieldResult<Vec<Product>> {
    let res = products::table.load::<Product>(conn);

    utils::graphql_translate(res)
}

pub fn get_product_by_id(conn: &PgConnection, id: i32) -> FieldResult<Product> {
    utils::graphql_translate(
        products::table
            .filter(products::columns::id.eq(id))
            .get_result::<Product>(conn),
    )
}

pub fn products_with_notification(conn: &PgConnection) -> FieldResult<Vec<Product>> {
    let res = products::table
        .inner_join(notifications::table.inner_join(users::table))
        .select(products::all_columns)
        .load(conn);

    utils::graphql_translate(res)
}

pub fn users_notified_of_product(conn: &PgConnection, product_id: i32) -> FieldResult<Vec<User>> {
    let res = products::table
        .inner_join(notifications::table.inner_join(users::table))
        .filter(notifications::columns::product_id.eq(product_id))
        .select(users::all_columns)
        .load(conn);

    utils::graphql_translate(res)
}

pub fn is_user_notified_about_product(
    conn: &PgConnection,
    user_id: i32,
    product_id: i32,
) -> FieldResult<bool> {
    let res: Result<Vec<User>, _> = products::table
        .inner_join(notifications::table.inner_join(users::table))
        .filter(notifications::columns::product_id.eq(product_id))
        .filter(notifications::columns::user_id.eq(user_id))
        .select(users::all_columns)
        .load(conn);

    let res = utils::graphql_translate(res)?;
    Ok(!res.is_empty())
}

pub fn get_collection_of_product(
    conn: &PgConnection,
    product_id: i32,
) -> FieldResult<Option<Collection>> {
    let res = collections::table
        .inner_join(collections_products_relation::table.inner_join(products::table))
        .filter(collections_products_relation::columns::product_id.eq(product_id))
        .select(collections::all_columns)
        .limit(1)
        .load::<Collection>(conn);

    if let Ok(v) = res {
        if v.is_empty() {
            Ok(None)
        } else {
            utils::graphql_translate(Ok(Some(v[0].clone())))
        }
    } else {
        utils::graphql_translate(Err(res.unwrap_err()))
    }
}
