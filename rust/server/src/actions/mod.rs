use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

mod class;
mod event;
mod user;

pub type DbResult<T> = Result<T, crate::error::ServiceErr>;

type Connection = ConnectionManager<PgConnection>;
pub type Pool = r2d2::Pool<Connection>;
