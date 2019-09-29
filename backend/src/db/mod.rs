use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager, Pool, PooledConnection},
};
use actix::{Actor, SyncContext};

pub type Conn = PgConnection;
pub type DbPool = Pool<ConnectionManager<Conn>>;
pub type PooledConn = PooledConnection<ConnectionManager<Conn>>;

#[derive(Clone)]
pub struct DbExecutor(pub DbPool);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl DbExecutor {
    pub fn new<S: Into<String>>(database_url: S) -> Self {
        let manager = ConnectionManager::<Conn>::new(database_url);
        let pool = r2d2::Pool::builder().build(manager)
            .expect("should build db connection pool");
        DbExecutor(pool)
    }
}
