extern crate diesel;

use diesel::prelude::*;

/* Comments older than 6 years are obsolete. */
//pub const SMS_COMMENT_DATE_LIMIT: u64 = 60 * 60 * 24 * 366 * 6;

pub fn establish_connection() -> SqliteConnection {
    let database_url = "database/sms_db_sqlite";
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
