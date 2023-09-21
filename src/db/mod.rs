use sqlite::{open, Connection, Error};
use std::result::Result;
pub mod entities;

pub use entities::*;

/// The function `get_database_connection` returns a connection to a SQLite database and asserts the
/// existence of required tables.
///
/// Returns:
///
/// The function `get_database_connection` returns a `Connection` object.
pub fn get_database_connection() -> Connection {
    let connection = open("./rss_rust.sqlite3").unwrap();
    let _ = assert_tables(&connection).expect("DB tables assertion failed");
    return connection;
}

/// The function `assert_tables` creates two tables (`rss_table` and `rss_entries`) if they do not
/// already exist in the database.
///
/// Arguments:
///
/// * `connection`: The `connection` parameter is of type `&Connection`, which represents a connection
/// to a database. It is used to execute SQL statements and interact with the database.
///
/// Returns:
///
/// The function `assert_tables` returns a `Result<(), Error>`.
fn assert_tables(connection: &Connection) -> Result<(), Error> {
    connection.execute(
        "
        CREATE TABLE IF NOT EXISTS rss_table (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            rss_url VARCHAR(250) NOT NULL,
            profile VARCHAR(250)
        );
        ",
    )?;

    connection.execute(
        "
        CREATE TABLE IF NOT EXISTS rss_entries (
            hash VARCHAR(250) PRIMARY KEY,
            title TEXT NOT NULL,
            publish_date DATE DEFAULT CURRENT_TIMESTAMP,
            link TEXT NOT NULL
        );
    ",
    )?;
    Ok(())
}
