use sqlite::Connection;
use std::vec::Vec;
use thread_safe::ThreadSafe;

use super::structs::GetManyOptions;

/// The `Connectable` trait is defining a method called `set_connection` that takes a mutable reference
/// to `self` and a reference to a `Connection` object. The `'c` and `'conn` lifetimes are used to
/// specify the lifetime of the references. This trait can be implemented by types that need to
/// establish a connection to a database and set that connection for future use.
pub trait Connectable<'c> {
    fn set_connection<'conn: 'c>(&mut self, connection: &'conn Connection);
}

/// The code defines a trait called `CrudAble` with two associated functions: `create` and `get_many`.

pub trait CrudAble<'a, T>
where
    T: Connectable<'a>,
{
    fn save<'c>(&'c self) -> &'c T;
    fn get_many<'conn>(
        connection: &ThreadSafe<&'conn Connection>,
        options: GetManyOptions,
    ) -> Vec<T>;
}
