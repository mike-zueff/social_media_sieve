use diesel::prelude::*;

const SMS_DB_PATH: &str = "database/sms_db_sqlite";

pub const SMS_DB_ERROR_QUERY: &str = "Database query error occured.";

/* Connect to the database. */
pub fn sms_db_conn_establish() -> SqliteConnection {
    SqliteConnection::establish(SMS_DB_PATH)
        .unwrap_or_else(|_| panic!("Can't connect to the database."))
}
