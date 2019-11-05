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

    let vk_api_tokens = file_to_vec("config/private_vk_api_tokens").expect(SMS_IO_ERROR_COMMON);

    println!(
        "VK API tokens:{}",
        vk_api_tokens
            .iter()
            .fold(String::new(), |acc, token| acc + "\n" + &token.to_string())
    );

    let conn = sms_db_conn_establish();

    let result: i64 = vor_obl_settlements
        .count()
        .get_result(&conn)
        .expect(SMS_DB_ERROR_QUERY);

    println!("Settlements count in the database: {}.", result);

    println!("SMS stopped.");
}
