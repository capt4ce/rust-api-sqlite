use anyhow::Result;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use diesel::SqliteConnection;
use diesel_migrations::embed_migrations;
use lazy_static::lazy_static;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

embed_migrations!();

lazy_static! {
    static ref POOL: Pool = {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found");
        let pool = Pool::builder()
            .build(ConnectionManager::new(database_url))
            .expect("Can not connect to DB");
        pool
    };
}

pub fn init() {
    lazy_static::initialize(&POOL);
    let conn = connection().expect("can not connect to DB for migration");
    embedded_migrations::run(&conn).unwrap();
}

pub fn connection() -> Result<PooledConnection<ConnectionManager<SqliteConnection>>> {
    let conn = POOL.get()?;
    Ok(conn)
}
