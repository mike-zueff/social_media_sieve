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

    let sms_current_vk_api_app_id = 0;
    let sms_current_vk_api_token = 0;

    let sms_vk_api_app_ids =
        file_to_vec("config/private_vk_api_app_ids").expect(SMS_IO_ERROR_COMMON);

    let sms_vk_api_tokens = file_to_vec("config/private_vk_api_tokens").expect(SMS_IO_ERROR_COMMON);

    println!(
        "VK API tokens:{}",
        sms_vk_api_tokens
            .iter()
            .fold(String::new(), |acc, token| acc
                + "\n- "
                + &token.to_string()
                + ".")
    );

    println!(
        "Current VK API token:\n{}.",
        sms_vk_api_tokens[sms_current_vk_api_token]
    );

    println!(
        "VK API app ids:{}",
        sms_vk_api_app_ids
            .iter()
            .fold(String::new(), |acc, app_id| acc
                + "\n- "
                + &app_id.to_string()
                + ".")
    );

    println!(
        "Current VK API app id: {}.",
        sms_vk_api_app_ids[sms_current_vk_api_app_id]
    );

    let sms_db_connection = sms_db_conn_establish();

    let result: i64 = vor_obl_settlements
        .count()
        .get_result(&sms_db_connection)
        .expect(SMS_DB_ERROR_QUERY);

    println!(
        "Voronezh Oblast settlements count in the database: {}.",
        result
    );

    println!("SMS stopped.");
}
