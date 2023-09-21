use crate::{
    core::{
        structs::GetManyOptions,
        traits::{Connectable, CrudAble},
    },
    rss::Rss,
};

use super::blog::RssEntry;
use sqlite::{Connection, State};
use std::fmt::Display;
use thread_safe::ThreadSafe;

pub struct RssEntity<'c> {
    pub id: u8,
    pub rss_url: String,
    pub profile: String,
    pub connection: Option<ThreadSafe<&'c Connection>>,
}

impl<'c> Display for RssEntity<'c> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(  id = {}\trss_url = {}\tprofile = {}\thas_connection = {}  )",
            self.id,
            self.rss_url,
            self.profile,
            self.connection.is_some()
        )
    }
}

/// The `impl<'c> Connectable<'c> for RssEntity<'c>` block is implementing the `Connectable` trait for
/// the `RssEntity` struct.
impl<'c> Connectable<'c> for RssEntity<'c> {
    fn set_connection<'conn: 'c>(&mut self, connection: &'conn Connection) {
        self.connection = Some(ThreadSafe::new(connection));
    }
}

impl<'c> CrudAble<'c, RssEntity<'c>> for RssEntity<'c> {
    /// The function creates a new entry in the database table for an RSS entity and returns a reference
    /// to the created entity.
    ///
    /// Arguments:
    ///
    /// * `entity`: The `entity` parameter is a reference to an `RssEntity` struct.
    ///
    /// Returns:
    ///
    /// a reference to the `RssEntity` object that was passed as an argument.
    fn save<'d>(&'d self) -> &'d RssEntity<'c> {
        if let Some(connection) = self.connection.clone() {
            // database insertion here
            let connection = connection.get_ref();
            let insert_query = "
            INSERT INTO rss_table (
                rss_url, profile
            )VALUES (
                ?, ?
            )
            ";
            let mut insert_statement = connection.prepare(insert_query).unwrap();
            insert_statement
                .bind_iter([(1, self.rss_url.as_str()), (2, self.profile.as_str())])
                .unwrap();

            match insert_statement.next() {
                Ok(_) => self,
                Err(_) => &self,
            }
        } else {
            panic!("No connection failed to save")
        }
    }

    /// The function `get_many` retrieves multiple rows from a database table named `rss_table` and
    /// returns them as a vector of `RssEntity` structs.
    ///
    /// Arguments:
    ///
    /// * `connection`: The `connection` parameter is a reference to a `Connection` object. It represents
    /// a connection to a database and is used to execute SQL queries and interact with the database.
    ///
    /// Returns:
    ///
    /// The function `get_many` returns a vector of `RssEntity` objects.
    #[allow(unused)]
    fn get_many<'conn>(
        connection: &ThreadSafe<&'conn Connection>,
        options: GetManyOptions,
    ) -> Vec<RssEntity<'c>> {
        let query = "SELECT * FROM rss_table";
        let connection = connection.get_ref();
        let mut statement = connection.prepare(query).unwrap();
        let mut rss_entity_vec: Vec<RssEntity> = vec![];
        while let Ok(State::Row) = statement.next() {
            rss_entity_vec.push(RssEntity {
                id: statement.read::<i64, _>("id").unwrap() as u8,
                rss_url: statement.read::<String, _>("rss_url").unwrap(),
                profile: statement.read::<String, _>("profile").unwrap(),
                connection: None,
            })
        }
        return rss_entity_vec;
    }
}

impl<'c> RssEntity<'c> {
    /// The function `sync` retrieves multiple RSS entities from a database connection, parses each RSS
    /// feed, and saves the entries into the database.
    ///
    /// Arguments:
    ///
    /// * `conn`: The `conn` parameter is a reference to an `Arc<Connection>` object. It is likely a
    /// connection to a database or some other data source that is used to retrieve and save data.
    pub async fn sync(connection: &ThreadSafe<&'c Connection>) -> () {
        let rss_vec = RssEntity::get_many(&connection, GetManyOptions::new());

        for rss in rss_vec {
            let mut rss = Rss::parse(String::from(rss.rss_url)).await.unwrap();

            for item in rss.get_items().iter() {
                let mut rss_entry = RssEntry::from(item.clone());
                rss_entry.connection = Some(connection.clone());
                rss_entry.save();
            }
        }
    }
}
