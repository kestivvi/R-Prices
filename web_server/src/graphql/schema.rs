use database::context::GraphQLContext;
use juniper::{EmptySubscription, RootNode};

use super::mutations::Mutation;
use super::queries::Query;

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<GraphQLContext>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}
