extern crate serde_json as json;

use std::fs::*;
use vkrs::auth::*;

/* Materials older than 6 years are obsolete. */
/* const SMS_MATERIAL_DATE_LIMIT: u32 = 60 * 60 * 24 * 366 * 6; */

/* const SMS_VK_API_GETCITIES_COUNT: u16 = 1000; */
const SMS_VK_API_TOKEN_ERROR_LOAD: &str = "VK API token load error occured.";
const SMS_VK_API_TOKEN_PATH: &str = "config/private_vk_api_current_token.json";
/* const SMS_VK_API_VOR_OBL_ID: u32 = 1023816; */

pub fn sms_vk_api_token_load() -> AccessToken {
    let token = File::open(SMS_VK_API_TOKEN_PATH)
        .ok()
        .and_then(|mut f| serde_json::from_reader(&mut f).ok());

    match token {
        Some(val) => val,
        None => panic!(SMS_VK_API_TOKEN_ERROR_LOAD),
    }
}
