pub mod loader;
pub mod mutations;
pub mod queries;

use std::fmt::Display;

use juniper::{graphql_object, FieldResult};

use crate::{
    context::GraphQLContext,
    diesel_schema::prices,
    models::{self, offer::Offer},
};

// define your enum
#[derive(Debug, Clone, Copy, PartialEq, diesel_derive_enum::DbEnum, juniper::GraphQLEnum)]
pub enum Availability {
    Available,
    TemporarilyUnavailable,
    Unavailable,
    PriceNotFound,
    SiteNotFound,
}

impl Display for Availability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Queryable, Clone, Debug)]
pub struct Price {
    pub id: i32,
    pub offer_id: i32,
    pub value: Option<f64>,
    pub created_at: chrono::NaiveDateTime,
    pub availability: Availability,
}

#[graphql_object(context = GraphQLContext)]
impl Price {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }

    pub fn created_at(&self) -> chrono::NaiveDateTime {
        self.created_at
    }

    pub fn offer(&self, context: &GraphQLContext) -> FieldResult<Offer> {
        let conn = &context.pool.get()?;
        models::offer::queries::offer_by_id(conn, self.offer_id)
    }

    pub fn availability(&self) -> Availability {
        self.availability
    }
}

// TODO: This is probably not needed for users outside
// The GraphQL input object for creating PRICESs
#[derive(juniper::GraphQLInputObject, Insertable)]
#[table_name = "prices"]
pub struct CreatePriceInput {
    pub offer_id: i32,
    pub value: Option<f64>,
    pub availability: Availability,
}
