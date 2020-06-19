use std::result::Result;
use r2d2;
use crate::GRConnection;
use crate::Error;

#[derive(Clone, Debug)]
pub struct ConnectionManager {
    url: String,
}

impl ConnectionManager {
    pub fn new<S: Into<String>>(url: S) -> ConnectionManager {
        ConnectionManager {
            url: url.into(),
        }
    }
}

impl r2d2::ManageConnection for ConnectionManager {
    type Connection = GRConnection;
    type Error = Error;

    fn connect(&self) -> Result<GRConnection, Error> {
        Ok(GRConnection::new(self.url.clone())?)
    }

    fn is_valid(&self, conn: &mut GRConnection) -> Result<(), Error> {
        match conn.is_connected() {
			true => return Ok(()),
			false => return Err("Connection broken".into())
		}
    }

    fn has_broken(&self, conn: &mut GRConnection) -> bool {
        conn.err()
    }
}