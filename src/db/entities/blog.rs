use digest::Digest;
use sqlite::{Connection, State};
use std::fmt::Display;
use thread_safe::ThreadSafe;

use sha2::Sha256;

use crate::{
    core::{
        structs::GetManyOptions,
        traits::{Connectable, CrudAble},
    },
    rss::dto::Item,
};

/// The `RssEntry` struct represents an RSS entry with a title, link, publish date, and an optional
/// connection.
///
/// Properties:
///
/// * `title`: A string representing the title of the RSS entry.
/// * `link`: A string representing the URL or link associated with the RSS entry.
/// * `publish_date`: The `publish_date` property is a string that represents the date when the RSS
/// entry was published.
/// * `connection`: The `connection` property is an optional field that holds a reference to a
/// `ThreadSafe` wrapper around a `&Connection` object. The `ThreadSafe` wrapper ensures that the
/// reference to the `Connection` object can be safely shared across multiple threads.
pub struct RssEntry<'c> {
    pub title: String,
    pub link: String,
    pub publish_date: String,
    pub connection: Option<ThreadSafe<&'c Connection>>,
}

/// The `impl<'c> Display for RssEntry<'c>` block is implementing the `Display` trait for the `RssEntry`
/// struct. This allows instances of `RssEntry` to be formatted as a string when using the `format!`
/// macro or the `println!` macro.
impl<'c> Display for RssEntry<'c> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Title: {}\nLink: {}\nDate: {}\n",
            self.title, self.link, self.publish_date
        )
    }
}

impl<'c> Connectable<'c> for RssEntry<'c> {
    fn set_connection<'conn: 'c>(&mut self, connection: &'conn Connection) {
        self.connection = Some(ThreadSafe::new(connection));
    }
}

impl<'c> CrudAble<'c, RssEntry<'c>> for RssEntry<'c> {
    /// The `save` function saves an RSS entry to a database if a database connection is available,
    /// using a hash of the entry's title as the primary key.
    ///
    /// Returns:
    ///
    /// a reference to an `RssEntry` object.
    fn save<'d>(&'d self) -> &'d RssEntry<'c> {
        // database insertion here
        match self.connection.clone() {
            Some(connection) => {
                let mut hasher = Sha256::new();
                let connection = connection.get_ref();
                hasher.update(self.title.clone());
                let result = hasher.finalize();
                let hash = format!("{:x}", result);

                let count_query = format!(
                    "
                    SELECT COUNT(*) AS \"count\" 
                    FROM rss_entries
                    WHERE hash = ?
                    LIMIT 1
                    "
                );
                let insert_query = format!(
                    "
                    INSERT INTO rss_entries (
                        hash, title,
                        publish_date, link
                    )VALUES(
                        ?, ?, CURRENT_TIMESTAMP, ?
                    )"
                );

                let mut count_statement = connection.prepare(count_query).unwrap();
                count_statement.bind((1, hash.as_str())).unwrap();

                let mut insert_statement = connection.prepare(insert_query).unwrap();
                insert_statement.bind((1, hash.as_str())).unwrap();
                insert_statement.bind((2, self.title.as_str())).unwrap();
                insert_statement.bind((3, self.link.as_str())).unwrap();

                match count_statement.next() {
                    Ok(State::Row) | Ok(State::Done) => {
                        match count_statement.read::<String, _>("count").as_deref() {
                            Ok("1") => &self,
                            Ok("0") | Ok(_) => {
                                return match insert_statement.next() {
                                    Ok(_) => self,
                                    Err(_) => self,
                                }
                            }

                            Err(_) => panic!("Insert into rss_entries failed"),
                        }
                    }
                    Err(_) => &self,
                }
            }
            None => panic!("DB connection not set"),
        }
    }

    /// The function creates a new entry in a database table if a matching entry does not already exist,
    /// and returns a reference to the created or existing entry.
    ///
    /// Arguments:
    ///
    /// * `entity`: The `entity` parameter is a reference to an `RssEntry` struct.
    ///
    /// Returns:
    ///
    /// a reference to an `RssEntry` object.

    /// The function retrieves multiple rows from a database table and returns them as a vector of
    /// RssEntry structs.
    ///
    /// Arguments:
    ///
    /// * `connection`: The `connection` parameter is a reference to a `Connection` object. It
    /// represents a connection to a database and is used to execute SQL queries and interact with the
    /// database.
    ///
    /// Returns:
    ///
    /// The function `get_many` returns a vector of `RssEntry` structs.
    fn get_many<'conn>(
        connection: &ThreadSafe<&'conn Connection>,
        options: GetManyOptions,
    ) -> Vec<RssEntry<'c>> {
        let connection = connection.get_ref();
        let query = "
            SELECT * FROM rss_entries 
            WHERE title LIKE ?
            LIMIT ?
            OFFSET ?
        ";

        let (regex, limit, offset) = options.as_prepared_tuple();
        let mut statement = connection.prepare(query).expect("Statement is not built");
        statement
            .bind_iter::<_, (_, &str)>([
                ((1, regex.as_str())),
                ((2, limit.as_str())),
                ((3, offset.as_str())),
            ])
            .expect("Binding failed");

        let mut rss_entity_vec: Vec<RssEntry> = vec![];
        while let Ok(State::Row) = statement.next() {
            rss_entity_vec.push(RssEntry {
                title: statement
                    .read::<String, _>("title")
                    .expect("TITLE NOT FOUND")
                    .to_string(),
                link: statement
                    .read::<String, _>("link")
                    .expect("LINK NOT FOUND")
                    .to_string(),
                publish_date: statement
                    .read::<String, _>("publish_date")
                    .expect("PUBLISH DATE NOT FOUND"),
                connection: None,
            })
        }
        return rss_entity_vec;
    }
}

/// The `impl<'c> Clone for RssEntry<'c>` block is implementing the `Clone` trait for the `RssEntry`
/// struct. This allows instances of `RssEntry` to be cloned, creating a new instance with the same
/// field values.
impl<'c> Clone for RssEntry<'c> {
    fn clone(&self) -> Self {
        RssEntry {
            title: self.title.clone(),
            link: self.link.clone(),
            publish_date: self.publish_date.clone(),
            connection: self.connection.clone(),
        }
    }
}

/// The `impl<'c> From<Item> for RssEntry<'c>` block is implementing the `From` trait for converting
/// from an `Item` struct to an `RssEntry` struct. This allows an `Item` object to be converted into an
/// `RssEntry` object using the `into()` function. The implementation specifies how the fields of the
/// `Item` struct are mapped to the fields of the `RssEntry` struct. In this case, the `title`, `link`,
/// and `publish_date` fields of the `Item` struct are assigned to the corresponding fields of the
/// `RssEntry` struct, and the `connection` field is set to `None`.
impl<'c> From<Item> for RssEntry<'c> {
    fn from(item: Item) -> Self {
        RssEntry {
            title: item.title,
            link: item.link,
            publish_date: item.publish_date,
            connection: None,
        }
    }
}
