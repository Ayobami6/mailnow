// this contains the connect_db method
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

// the DBPool type
pub type DBPool = Pool<ConnectionManager<PgConnection>>;

pub fn connect_db(db_url: &str) -> DBPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool. 🚨")
}
