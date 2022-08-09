use database::models::collection::Collection;
use juniper::{FieldError, FieldResult};

use database::models;
use database::{
    context::GraphQLContext,
    models::{offer::Offer, price::Price, product::Product},
};

pub struct Query;

#[juniper::graphql_object(Context = GraphQLContext)]
impl Query {
    //////////////////////////////////////////////////////////////////////////
    // PRICE

    pub fn all_prices(context: &GraphQLContext) -> FieldResult<Vec<Price>> {
        let conn = &context.pool.get()?;
        models::price::queries::all_prices(conn)
    }

    pub fn get_price_by_id(
        context: &GraphQLContext,
        id: i32,
    ) -> FieldResult<Option<Price>> {
        let conn = &context.pool.get()?;
        models::price::queries::get_price_by_id(conn, id)
    }

    pub fn get_price_of_offer_id(
        context: &GraphQLContext,
        offer_id: i32,
    ) -> FieldResult<Vec<Price>> {
        let conn = &context.pool.get()?;
        models::price::queries::get_price_of_offer_id(conn, offer_id)
    }

    //////////////////////////////////////////////////////////////////////////
    // OFFER

    pub fn all_offers(context: &GraphQLContext) -> FieldResult<Vec<Offer>> {
        let conn = &context.pool.get()?;
        models::offer::queries::all_offers(conn)
    }

    //////////////////////////////////////////////////////////////////////////
    // PRODUCT

    pub fn all_products(context: &GraphQLContext) -> FieldResult<Vec<Product>> {
        let conn = context.pool.get()?;
        models::product::queries::all_products(&conn)
    }

    pub fn get_product_by_id(context: &GraphQLContext, id: i32) -> FieldResult<Product> {
        let conn = context.pool.get()?;

        let collection =
            database::models::product::queries::get_collection_of_product(&conn, id)?;

        let product = database::models::product::queries::get_product_by_id(&conn, id);

        if collection.is_none() || collection.as_ref().unwrap().public {
            return product;
        } else {
            if let Some(user_id) = context.user_id {
                // Authorization
                let owner =
                    database::models::collection::queries::get_owner_of_collection(
                        &conn,
                        collection.unwrap().id,
                    )?;

                if owner.id == user_id {
                    return product;
                } else {
                    return Err(FieldError::from("You're not authorized to do this!\nThis product belongs to a private collection and you don't own it"));
                }
            } else {
                return Err(FieldError::from("You're not logged in!"));
            }
        }
    }

    //////////////////////////////////////////////////////////////////////////
    // COLLECTION

    pub fn all_collections(context: &GraphQLContext) -> FieldResult<Vec<Collection>> {
        let conn = &context.pool.get()?;
        database::models::collection::queries::all_collections(conn)
    }

    pub fn all_public_collections(
        context: &GraphQLContext,
    ) -> FieldResult<Vec<Collection>> {
        let conn = &context.pool.get()?;
        database::models::collection::queries::all_public_collections(conn)
    }

    pub fn get_collection_by_id(
        context: &GraphQLContext,
        collection_id: i32,
    ) -> FieldResult<Collection> {
        let conn = &context.pool.get()?;

        let collection = database::models::collection::queries::get_collection_by_id(
            conn,
            collection_id,
        )?;

        if collection.public {
            return Ok(collection);
        }

        if let Some(user_id) = context.user_id {
            // Authorization
            let owner = database::models::collection::queries::get_owner_of_collection(
                conn,
                collection_id,
            )?;

            if owner.id == user_id {
                return Ok(collection);
            } else {
                return Err(FieldError::from("You're not authorized to do this!\nThis collection is set as private and you don't own it"));
            }
        } else {
            return Err(FieldError::from("You're not logged in!"));
        }
    }

    pub fn get_my_collections(context: &GraphQLContext) -> FieldResult<Vec<Collection>> {
        let conn = &context.pool.get()?;
        match context.user_id {
            Some(v) => {
                database::models::collection::queries::get_collections_of_user(conn, v)
            }
            None => Err(FieldError::from("You're not logged in")),
        }
    }

    pub fn am_i_owner_of_collection(
        context: &GraphQLContext,
        collection_id: i32,
    ) -> FieldResult<bool> {
        let conn = &context.pool.get()?;

        match context.user_id {
            Some(user_id) => {
                let owner =
                    database::models::collection::queries::get_owner_of_collection(
                        conn,
                        collection_id,
                    )?;

                if owner.id == user_id {
                    return Ok(true);
                } else {
                    return Ok(false);
                };
            }
            None => Ok(false),
        }
    }
}
