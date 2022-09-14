use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use juniper::FieldResult;

use crate::{
    diesel_schema::users,
    models::utils::{self},
};

use super::{RegisterLoginUserInput, User};

pub fn get_user_by_id(conn: &PgConnection, user_id: i32) -> FieldResult<User> {
    let res = users::table
        .filter(users::columns::id.eq(user_id))
        .get_result::<User>(conn);
    utils::graphql_translate(res)
}

pub fn get_user_by_email(conn: &PgConnection, user_email: &str) -> FieldResult<User> {
    let res = users::table
        .filter(users::columns::email.eq(user_email))
        .get_result::<User>(conn);
    utils::graphql_translate(res)
}

pub fn get_user_by_name(conn: &PgConnection, user_name: &str) -> FieldResult<User> {
    let res = users::table
        .filter(users::columns::name.eq(user_name))
        .get_result::<User>(conn);
    utils::graphql_translate(res)
}

pub fn login_user(conn: &PgConnection, login_input: RegisterLoginUserInput) -> FieldResult<User> {
    let res = users::table
        .filter(users::columns::email.eq(login_input.email))
        .filter(users::columns::password.eq(login_input.password))
        .get_result::<User>(conn);
    utils::graphql_translate(res)
}

pub fn is_that_user_have_that_passwd(conn: &PgConnection, user_id: i32, passwd: &str) -> bool {
    let res = users::table
        .filter(users::columns::id.eq(user_id))
        .get_result::<User>(conn);

    let user = match res {
        Ok(v) => v,
        Err(_) => return false,
    };

    user.password == passwd
}
