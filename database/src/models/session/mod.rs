pub mod mutations;
pub mod queries;

use crate::diesel_schema::sessions;

#[derive(Queryable, Clone, Debug)]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
}

#[derive(Insertable, Clone, Debug)]
#[table_name = "sessions"]
pub struct SessionInput {
    pub user_id: i32,
}
