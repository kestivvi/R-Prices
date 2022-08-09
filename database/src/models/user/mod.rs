pub mod mutations;
pub mod queries;

use serde::Deserialize;
use std::fmt::Display;

use crate::{context::GraphQLContext, diesel_schema::users};

#[derive(Queryable, Clone, Debug, serde::Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[juniper::graphql_object(context = GraphQLContext)]
impl User {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    // pub fn email(&self) -> &str {
    //     &self.email
    // }

    // pub fn password(&self) -> &str {
    //     &self.password
    // }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

// TODO: This is probably not needed for users outside
// The GraphQL input object for creating PRICESs
#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct RegisterLoginUserInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct NewName {
    pub new_name: String,
}

#[derive(serde::Deserialize)]
pub struct NewEmail {
    pub new_email: String,
}

#[derive(serde::Deserialize)]
pub struct PasswordChange {
    pub old_password: String,
    pub new_password: String,
}
