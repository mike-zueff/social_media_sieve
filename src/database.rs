use crate::schema::vor_obl_settlements::dsl::*;
use diesel::prelude::*;
use vkrs::api::*;
use vkrs::auth::*;
use vkrs::*;

const SMS_DB_PATH: &str = "database/sms_db_sqlite";

pub const SMS_DB_ERROR_QUERY: &str = "Database query error occured.";

/* Connect to the database. */
pub fn sms_db_conn_establish() -> SqliteConnection {
    SqliteConnection::establish(SMS_DB_PATH).expect("Can't connect to the database.")
}

/* Check and fetch settlements. */
pub fn sms_db_settl_fetch(conn: &SqliteConnection, api: &Client, token: &AccessToken) {
    let result: i64 = vor_obl_settlements
        .count()
        .get_result(conn)
        .expect(SMS_DB_ERROR_QUERY);

    if result == 0 {
        let audio_result = api
            .call(
                Some(token),
                audio::Search::new()
                    .q("commander")
                    .performer_only(false)
                    .count(200),
            )
            .unwrap();

        for audio in &audio_result.items {
            println!("{:?}", audio);
        }
    } else {
        println!(
            "Number of Voronezh Oblast settlements in the database: {}.",
            result
        );
    }
}
