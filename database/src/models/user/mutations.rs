use diesel::prelude::*;
use diesel::PgConnection;
use diesel::RunQueryDsl;
use juniper::FieldResult;

use super::RegisterLoginUserInput;
use super::User;
use crate::diesel_schema::users;
use crate::models::utils;

pub fn register_user(conn: &PgConnection, new_user: RegisterLoginUserInput) -> FieldResult<User> {
    let res = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn);

    utils::graphql_translate(res)
}

pub fn change_name(conn: &PgConnection, user_id: i32, new_name: &str) -> FieldResult<()> {
    let res = diesel::update(users::table)
        .filter(users::columns::id.eq(user_id))
        .set(users::columns::name.eq(new_name))
        .execute(conn);

    utils::graphql_translate(res.map(|_| ()))
}

pub fn change_email(conn: &PgConnection, user_id: i32, new_email: &str) -> FieldResult<()> {
    let res = diesel::update(users::table)
        .filter(users::columns::id.eq(user_id))
        .set(users::columns::email.eq(new_email))
        .execute(conn);

    utils::graphql_translate(res.map(|_| ()))
}

pub fn change_password(conn: &PgConnection, user_id: i32, new_password: &str) -> FieldResult<()> {
    let res = diesel::update(users::table)
        .filter(users::columns::id.eq(user_id))
        .set(users::columns::password.eq(new_password))
        .execute(conn);

    utils::graphql_translate(res.map(|_| ()))
}
