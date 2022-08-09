use super::Session;
use crate::diesel_schema::sessions;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};

pub fn get_user_id(conn: &PgConnection, session_id: i32) -> Option<i32> {
    sessions::table
        .find(session_id)
        .first::<Session>(conn)
        .ok()
        .map(|v| v.user_id)
}
