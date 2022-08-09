use diesel::ExpressionMethods;
use diesel::PgConnection;
use diesel::RunQueryDsl;
use juniper::FieldResult;

use crate::diesel_schema;
use crate::diesel_schema::collections_products_relation;
use crate::diesel_schema::notifications;
use crate::diesel_schema::offers;
use crate::diesel_schema::products;
use crate::diesel_schema::products_offers_relation;
use crate::models::offer::CreateOfferInput;
use crate::models::offer::Offer;
use crate::models::product::{Product, ProductInputDiesel};
use crate::models::utils;

use super::CollectionProductRelation;
use super::CreateNotificationRelation;
use super::CreateProductInput;

pub fn create_product(
    conn: &PgConnection,
    new_product: CreateProductInput,
) -> FieldResult<Product> {
    let product_input = ProductInputDiesel {
        name: new_product.name,
        description: new_product.description,
    };

    let res = diesel::insert_into(products::table)
        .values(&product_input)
        .get_result::<Product>(conn);
    let res = utils::graphql_translate(res);

    if res.is_err() {
        return res;
    }

    let collection_product_relation_input = CollectionProductRelation {
        collection_id: new_product.collection_id,
        product_id: res.as_ref().unwrap().id,
    };

    diesel::insert_into(collections_products_relation::table)
        .values(&collection_product_relation_input)
        .execute(conn)?;

    res
}

pub fn delete_product(conn: &PgConnection, id: i32) -> FieldResult<Product> {
    let res = diesel::delete(products::table)
        .filter(products::columns::id.eq(id))
        .get_result(conn);

    utils::graphql_translate(res)
}

pub fn update_notification(
    conn: &PgConnection,
    product_id: i32,
    user_id: i32,
    new_value: bool,
) -> bool {
    if new_value {
        diesel::insert_into(diesel_schema::notifications::table)
            .values(CreateNotificationRelation {
                product_id,
                user_id,
            })
            .execute(conn)
            .is_ok()
    } else {
        diesel::delete(diesel_schema::notifications::table)
            .filter(notifications::columns::product_id.eq(product_id))
            .filter(notifications::columns::user_id.eq(user_id))
            .execute(conn)
            .is_ok()
    }
}

pub fn rename(conn: &PgConnection, product_id: i32, new_value: String) -> FieldResult<Product> {
    let res = diesel::update(products::table)
        .filter(products::columns::id.eq(product_id))
        .set(products::columns::name.eq(new_value))
        .get_result(conn);

    utils::graphql_translate(res)
}

pub fn add_offer(conn: &PgConnection, product_id: i32, new_offer_url: &str) -> FieldResult<Offer> {
    let existing_offer = crate::models::offer::queries::get_offer_by_url(conn, new_offer_url);

    match existing_offer {
        Some(existing_offer) => {
            diesel::insert_into(products_offers_relation::table)
                .values((
                    diesel_schema::products_offers_relation::dsl::product_id.eq(product_id),
                    diesel_schema::products_offers_relation::dsl::offer_id.eq(existing_offer.id),
                ))
                .execute(conn)?;

            utils::graphql_translate(Ok(existing_offer))
        }
        None => {
            let res = diesel::insert_into(offers::table)
                .values(CreateOfferInput {
                    url: new_offer_url.to_owned(),
                })
                .get_result(conn);

            if res.is_err() {
                return utils::graphql_translate(res);
            }

            diesel::insert_into(products_offers_relation::table)
                .values((
                    diesel_schema::products_offers_relation::dsl::product_id.eq(product_id),
                    diesel_schema::products_offers_relation::dsl::offer_id
                        .eq(res.as_ref().unwrap().id),
                ))
                .execute(conn)?;

            return utils::graphql_translate(res);
        }
    }
}
