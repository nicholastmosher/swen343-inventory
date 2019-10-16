//! Defines an Actor for handling events that need access to the database.
//!
//! In the Actor model, each Actor is an object or entity which owns certain
//! resources and which can handle requests in the form of messages.
//! The runtime has the freedom to instantiate as many or as few actors as
//! are needed to handle the number of messages being sent to the inbox.
//!
//! In this module, we define the `DbExecutor` actor which owns a pooled
//! connection to the database. Messages sent to the `DbExecutor` represent
//! data that should be inserted or updated, or queries that should be
//! executed. Messages for `DbExecutor` and handlers for these messages are
//! defined in the submodules of this module.

use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager, Pool, PooledConnection},
};
use actix::{Actor, SyncContext};

pub mod items;
pub mod boxes;
pub mod pallets;
pub mod warehouses;

pub type Conn = PgConnection;
pub type DbPool = Pool<ConnectionManager<Conn>>;
pub type PooledConn = PooledConnection<ConnectionManager<Conn>>;

/// An Actor which interacts with the database through the DbPool.
#[derive(Clone)]
pub struct DbExecutor(pub DbPool);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl DbExecutor {

    /// Given a database URL, constructs a `DbExecutor` attached to that db.
    ///
    /// The `DbConn` held by the `DbExecutor` is really a handle to the
    /// underlying database pool. If the `DbExecutor` were to be created
    /// multiple times, each instance of it would simply have an atomic
    /// reference into the database pool. This allows the actor system to
    /// instantiate many instances of `DbExecutor` which can share the workload
    /// of messages, but which share the pooled available resources.
    pub fn new<S: Into<String>>(database_url: S) -> Self {
        let manager = ConnectionManager::<Conn>::new(database_url);
        let pool = r2d2::Pool::builder().build(manager)
            .expect("should build db connection pool");
        DbExecutor(pool)
    }
}
