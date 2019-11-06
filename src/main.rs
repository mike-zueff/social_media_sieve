#[macro_use]
extern crate diesel;

mod database;
use database::*;

mod schema;
use schema::vor_obl_settlements::dsl::*;

use diesel::prelude::*;

/* SMS entry point. */
fn main() {
    println!("SMS started.");

    let sms_db_conn = sms_db_conn_establish();

    let result: i64 = vor_obl_settlements
        .count()
        .get_result(&sms_db_conn)
        .expect(SMS_DB_ERROR_QUERY);

    if result == 0 {
        println!(
            "Number of Voronezh Oblast settlements in the database: {}.",
            result
        );
    } else {
        println!(
            "Number of Voronezh Oblast settlements in the database: {}.",
            result
        );
    }

    println!("SMS stopped.");
}
