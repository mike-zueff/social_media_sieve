#[macro_use]
extern crate diesel;

mod database;
mod schema;

use database::*;

/* SMS entry point. */
fn main() {
    println!("SMS started.");

    let sms_db_conn = sms_db_conn_establish();
    sms_db_settl_fetch(&sms_db_conn);

    println!("SMS stopped.");
}
