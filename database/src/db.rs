use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

// The Postgres-specific connection pool managing all database connections.
pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_pool(db_url: &str) -> PostgresPool {
    let mgr = ConnectionManager::<PgConnection>::new(db_url);

    r2d2::Pool::builder()
        .min_idle(Some(1))
        .max_size(30)
        .build(mgr)
        .map_err(|e| {
            log::error!(
                "Error while building connection to database: {}",
                e.to_string()
            );
        })
        .expect("could not build connection pool") // TODO: handle errors
}
