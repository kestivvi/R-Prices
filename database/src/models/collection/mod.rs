pub mod mutations;
pub mod queries;

use juniper::FieldResult;
use serde::{Deserialize, Serialize};

use crate::context::GraphQLContext;
use crate::diesel_schema::*;
use crate::models::product::Product;
use crate::models::user::User;

// #[derive(Queryable, juniper::GraphQLObject)]
#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub user_id: i32,
    pub public: bool,
}

#[juniper::graphql_object(context = GraphQLContext)]
impl Collection {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn public(&self) -> bool {
        self.public
    }

    pub fn owner(&self, context: &GraphQLContext) -> FieldResult<User> {
        let conn = &context.pool.get()?;
        super::user::queries::get_user_by_id(conn, self.user_id)
    }

    pub fn products(&self, context: &GraphQLContext) -> FieldResult<Vec<Product>> {
        let conn = &context.pool.get()?;
        super::collection::queries::get_products_of_collection(conn, self.id)
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct CreateCollectionInput {
    pub name: String,
    pub description: Option<String>,
    pub public: bool,
}

#[derive(Insertable)]
#[table_name = "collections"]
pub struct CreateCollectionDiesel {
    pub name: String,
    pub description: Option<String>,
    pub user_id: i32,
    pub public: bool,
}

// joinable!(collections_products_relation -> collections (collection_id));
joinable!(collections_products_relation -> products (product_id));
joinable!(collections -> users (user_id));
