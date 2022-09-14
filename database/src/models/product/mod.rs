pub mod mutations;
pub mod queries;

use juniper::FieldResult;

use crate::diesel_schema::*;
use crate::models::user::User;
use crate::{context::GraphQLContext, models::offer::Offer};

use super::collection::Collection;

// #[derive(Queryable, juniper::GraphQLObject)]
#[derive(Queryable, Debug)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[juniper::graphql_object(context = GraphQLContext)]
impl Product {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn collection(&self, context: &GraphQLContext) -> FieldResult<Option<Collection>> {
        let conn = &context.pool.get().unwrap();
        crate::models::product::queries::get_collection_of_product(conn, self.id)
    }

    pub fn notification(&self, context: &GraphQLContext) -> Option<bool> {
        let conn = &context.pool.get().unwrap();
        if let Some(user_id) = context.user_id {
            queries::is_user_notified_about_product(conn, user_id, self.id).ok()
        } else {
            None
        }
    }

    // TODO: get notification of a product
    pub fn notified_users(&self, context: &GraphQLContext) -> Vec<User> {
        let conn = &context.pool.get().unwrap();
        queries::users_notified_of_product(conn, self.id).unwrap()
    }

    pub async fn offers(&self, context: &GraphQLContext) -> Vec<Offer> {
        context.offer_loader.load(self.id).await
    }
}

#[derive(juniper::GraphQLInputObject, Debug)]
pub struct CreateProductInput {
    pub name: String,
    pub description: Option<String>,
    pub collection_id: i32,
}

#[derive(Insertable)]
#[table_name = "products"]
pub struct ProductInputDiesel {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Insertable)]
#[table_name = "collections_products_relation"]
pub struct CollectionProductRelation {
    pub collection_id: i32,
    pub product_id: i32,
}

#[derive(Insertable)]
#[table_name = "notifications"]
pub struct CreateNotificationRelation {
    pub product_id: i32,
    pub user_id: i32,
}

#[derive(Identifiable, Queryable, Associations)]
#[table_name = "notifications"]
#[belongs_to(User, foreign_key = "user_id")]
#[belongs_to(Product, foreign_key = "product_id")]
pub struct NotificationRelation {
    pub id: i32,
    pub product_id: i32,
    pub user_id: i32,
}

joinable!(notifications -> products (product_id));
joinable!(notifications -> users (user_id));
