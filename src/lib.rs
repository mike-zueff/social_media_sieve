use std::fs;

use std::io;
use std::io::*;

/* Comments older than 6 years are obsolete. */
/* pub const SMS_COMMENT_DATE_LIMIT: u32 = 60 * 60 * 24 * 366 * 6; */

pub const SMS_DB_ERROR_QUERY: &str = "Database query error occured.";
pub const SMS_DB_PATH: &str = "database/sms_db_sqlite";
pub const SMS_IO_ERROR_COMMON: &str = "I/O error occured.";

/* Collect lines from a file. */
pub fn file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let file = fs::File::open(filename)?;
    let buf_reader = BufReader::new(file);

    Ok(buf_reader.lines().filter_map(io::Result::ok).collect())
}
