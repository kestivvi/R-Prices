use super::SessionInput;
use crate::diesel_schema::sessions;
use diesel::prelude::*;
use diesel::{PgConnection, RunQueryDsl};

pub fn insert(conn: &PgConnection, user_id: i32) -> Result<i32, diesel::result::Error> {
    diesel::insert_into(sessions::table)
        .values(SessionInput { user_id })
        .returning(sessions::columns::id)
        .get_result(conn)
}

pub fn remove_by_session_id(conn: &PgConnection, session_id: i32) {
    diesel::delete(sessions::table)
        .filter(sessions::columns::id.eq(session_id))
        .execute(conn)
        .ok();
}

pub fn remove_by_user_id(conn: &PgConnection, user_id: i32) {
    diesel::delete(sessions::table)
        .filter(sessions::columns::user_id.eq(user_id))
        .execute(conn)
        .ok();
}
