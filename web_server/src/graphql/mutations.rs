use juniper::{FieldError, FieldResult};

use database::{
    context::GraphQLContext,
    models::{
        self,
        collection::{Collection, CreateCollectionDiesel, CreateCollectionInput},
        offer::{AddOfferInput, Offer},
        price::{CreatePriceInput, Price},
        product::{CreateProductInput, Product},
    },
};

pub struct Mutation;

#[juniper::graphql_object(Context = GraphQLContext)]
impl Mutation {
    //////////////////////////////////////////////////////////////////////////
    // PRICE

    #[graphql(name = "createPrice")]
    pub fn create_price(
        context: &GraphQLContext,
        input: CreatePriceInput,
    ) -> FieldResult<Price> {
        let conn = &context.pool.get()?;
        models::price::mutations::create_price(conn, input)
    }

    //////////////////////////////////////////////////////////////////////////
    // OFFER

    // pub fn create_offer(
    //     context: &GraphQLContext,
    //     input: CreateOfferInput,
    // ) -> FieldResult<Offer> {
    //     let conn = &context.pool.get()?;
    //     models::offer::mutations::create_offer(conn, input)
    // }

    pub fn change_url_of_offer(
        context: &GraphQLContext,
        id: i32,
        new_value: String,
    ) -> FieldResult<Offer> {
        let conn = &context.pool.get()?;
        models::offer::mutations::change_url(conn, id, new_value)
    }

    pub fn delete_offer(context: &GraphQLContext, id: i32) -> FieldResult<Offer> {
        let conn = &context.pool.get()?;
        models::offer::mutations::delete_offer(conn, id)
    }

    //////////////////////////////////////////////////////////////////////////
    // PRODUCT

    pub fn create_product(
        context: &GraphQLContext,
        input: CreateProductInput,
    ) -> FieldResult<Product> {
        let conn = &context.pool.get()?;

        if let Some(user_id) = context.user_id {
            // Authorization
            let owner = database::models::collection::queries::get_owner_of_collection(
                conn,
                input.collection_id,
            )?;

            if owner.id == user_id {
                database::models::product::mutations::create_product(conn, input)
            } else {
                Err(FieldError::from("You're not authorized to do this!"))
            }
        } else {
            Err(FieldError::from("You're not logged in!"))
        }
    }

    pub fn delete_product(context: &GraphQLContext, id: i32) -> FieldResult<Product> {
        let conn = &context.pool.get()?;

        let collection =
            database::models::product::queries::get_collection_of_product(&conn, id)?;

        if collection.is_none() {
            return Err(FieldError::from("You're not authorized to do this!\nThis product doesn't belong to collections of yours"));
        } else {
            if let Some(user_id) = context.user_id {
                // Authorization
                let owner =
                    database::models::collection::queries::get_owner_of_collection(
                        &conn,
                        collection.unwrap().id,
                    )?;

                if owner.id == user_id {
                    return models::product::mutations::delete_product(conn, id);
                } else {
                    return Err(FieldError::from("You're not authorized to do this!\nThis product belongs to a private collection and you don't own it"));
                }
            } else {
                return Err(FieldError::from("You're not logged in!"));
            }
        }
    }

    pub fn update_notification_of_product(
        context: &GraphQLContext,
        product_id: i32,
        new_value: bool,
    ) -> FieldResult<Product> {
        let conn = &context.pool.get()?;

        if let Some(user_id) = context.user_id {
            match models::product::mutations::update_notification(
                conn, product_id, user_id, new_value,
            ) {
                true => models::product::queries::get_product_by_id(&conn, product_id),
                false => Err(FieldError::from("Some error updating notification")),
            }
        } else {
            Err(FieldError::from("You're not logged in!"))
        }
    }

    pub fn rename_product(
        context: &GraphQLContext,
        id: i32,
        new_value: String,
    ) -> FieldResult<Product> {
        let conn = &context.pool.get()?;
        models::product::mutations::rename(conn, id, new_value)
    }

    pub fn add_offer_to_product(
        context: &GraphQLContext,
        input: AddOfferInput,
    ) -> FieldResult<Offer> {
        if let Err(error) = url::Url::parse(&input.url) {
            return Err(FieldError::from(format!(
                "Given url for the new offer is not valid!\n{}",
                error
            )));
        };

        let conn = &context.pool.get()?;

        let collection = database::models::product::queries::get_collection_of_product(
            &conn,
            input.product_id,
        )?;

        if collection.is_none() {
            return Err(FieldError::from("You're not authorized to do this!\nThis product doesn't belong to collections of yours"));
        }

        if let Some(user_id) = context.user_id {
            // Authorization
            let owner = database::models::collection::queries::get_owner_of_collection(
                &conn,
                collection.unwrap().id,
            )?;

            if owner.id == user_id {
                return models::product::mutations::add_offer(
                    &conn,
                    input.product_id,
                    &input.url,
                );
            } else {
                return Err(FieldError::from("You're not authorized to do this!\nThis product belongs to a private collection and you don't own it"));
            }
        } else {
            return Err(FieldError::from("You're not logged in!"));
        }
    }

    //////////////////////////////////////////////////////////////////////////
    // COLLECTION

    pub fn create_collection(
        context: &GraphQLContext,
        new_collection: CreateCollectionInput,
    ) -> FieldResult<Collection> {
        let conn = &context.pool.get()?;

        if let Some(user_id) = context.user_id {
            let new_collection = CreateCollectionDiesel {
                name: new_collection.name,
                description: new_collection.description,
                public: new_collection.public,
                user_id,
            };
            database::models::collection::mutations::create_collection(
                conn,
                new_collection,
            )
        } else {
            Err(FieldError::from("You're not logged in!"))
        }
    }

    pub fn delete_collection(
        context: &GraphQLContext,
        collection_id: i32,
    ) -> FieldResult<Collection> {
        let conn = &context.pool.get()?;

        if let Some(user_id) = context.user_id {
            // Authorization
            let owner = database::models::collection::queries::get_owner_of_collection(
                conn,
                collection_id,
            )?;

            if owner.id == user_id {
                database::models::collection::mutations::delete(conn, collection_id)
            } else {
                Err(FieldError::from("You're not authorized to do this!"))
            }
        } else {
            Err(FieldError::from("You're not logged in!"))
        }
    }

    pub fn rename_collection(
        context: &GraphQLContext,
        collection_id: i32,
        new_name: String,
    ) -> FieldResult<Collection> {
        let conn = &context.pool.get()?;

        if let Some(user_id) = context.user_id {
            // Authorization
            let owner = database::models::collection::queries::get_owner_of_collection(
                conn,
                collection_id,
            )?;

            if owner.id == user_id {
                database::models::collection::mutations::rename(
                    conn,
                    collection_id,
                    new_name,
                )
            } else {
                Err(FieldError::from("You're not authorized to do this!"))
            }
        } else {
            Err(FieldError::from("You're not logged in!"))
        }
    }

    pub fn change_collection_description(
        context: &GraphQLContext,
        collection_id: i32,
        user_id: i32,
        new_description: String,
    ) -> FieldResult<Collection> {
        let conn = &context.pool.get()?;

        // Authorization
        let owner = database::models::collection::queries::get_owner_of_collection(
            conn,
            collection_id,
        )?;

        if owner.id == user_id {
            database::models::collection::mutations::change_description(
                conn,
                collection_id,
                new_description,
            )
        } else {
            Err(FieldError::from("Not authorized"))
        }
    }

    pub fn change_visibility_of_collection(
        context: &GraphQLContext,
        collection_id: i32,
        public: bool,
    ) -> FieldResult<Collection> {
        let conn = &context.pool.get()?;

        if let Some(user_id) = context.user_id {
            // Authorization
            let owner = database::models::collection::queries::get_owner_of_collection(
                conn,
                collection_id,
            )?;

            if owner.id == user_id {
                database::models::collection::mutations::change_visibility(
                    conn,
                    collection_id,
                    public,
                )
            } else {
                Err(FieldError::from("You're not authorized to do this!"))
            }
        } else {
            Err(FieldError::from("You're not logged in!"))
        }
    }
}
