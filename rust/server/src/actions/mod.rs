use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

pub mod class;
pub mod event;
pub mod user;

type Connection = ConnectionManager<PgConnection>;
pub type Pool = r2d2::Pool<Connection>;
