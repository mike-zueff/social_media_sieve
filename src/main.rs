#[macro_use]
extern crate diesel;

mod database;
mod schema;

use database::*;
use sms::*;
use vkrs::api::*;

/* SMS entry point. */
fn main() {
    println!("SMS started.");

    let sms_db_conn = sms_db_conn_establish();
    let vk_api = Client::new();
    let vk_api_token = sms_vk_api_token_load();

    sms_db_settl_fetch(&sms_db_conn, &vk_api, &vk_api_token);

    println!("SMS stopped.");
}
