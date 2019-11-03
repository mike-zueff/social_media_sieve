use rusqlite::*;
use std::*;

/* SMS entry point. */
fn main() {
    println!("SMS started.");

    match sms_db_init() {
        Ok(_) => println!("Database initialization succeeded."),
        Err(_) => panic!("Database initialization failed."),
    }

    println!("SMS stopped.");
}

/* Database initialization. */
fn sms_db_init() -> Result<()> {
    let _ = fs::create_dir("database");
    let sms_db_conn = Connection::open("database/sms_db_sqlite")?;

    let _ = sms_db_conn.execute(
        "CREATE TABLE if not exists vor_obl_settlements (
            id integer primary key,
            settlement_id integer,
            settlement_title text
        )", NO_PARAMS);

    Ok(())
}
