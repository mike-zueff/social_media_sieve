use diesel::prelude::*;
use sms::*;

/* Connect to the database. */
pub fn sms_db_conn_establish() -> SqliteConnection {
    SqliteConnection::establish(SMS_DB_PATH)
        .unwrap_or_else(|_| panic!("Can't connect to the database."))
}
