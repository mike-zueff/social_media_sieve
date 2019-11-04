#[macro_use]
extern crate diesel;

mod database;
use database::*;

mod schema;
use schema::vor_obl_settlements::dsl::*;

use diesel::prelude::*;
use sms::*;

/* SMS entry point. */
fn main() {
    println!("SMS started.");

    let conn = sms_db_conn_establish();

    let result: i64 = vor_obl_settlements
        .count()
        .get_result(&conn)
        .expect(SMS_DB_ERROR_QUERY);

    println!("Settlements count in the database: {}.", result);

    println!("SMS stopped.");
}
