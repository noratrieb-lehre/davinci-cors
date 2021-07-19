use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ServiceErr {
    ConnectionNotFound(r2d2::Error),
    DbActionFailed(diesel::result::Error),
}

impl std::error::Error for ServiceErr {}

impl Display for ServiceErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ServiceErr::ConnectionNotFound(err) => format!("Db Connection not found: {}", err),
                ServiceErr::DbActionFailed(err) => format!("Db Action Failed: {}", err),
            }
        )
    }
}

impl From<diesel::result::Error> for ServiceErr {
    fn from(err: diesel::result::Error) -> Self {
        Self::DbActionFailed(err)
    }
}

impl From<r2d2::Error> for ServiceErr {
    fn from(err: r2d2::Error) -> Self {
        Self::ConnectionNotFound(err)
    }
}
