//! This module contains code related to the database connection.
//!
//! See [`crate::migrator`] for the migrations.
//! See [`crate::entity`] for the entities.

use std::time::Duration;

use log::LevelFilter;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use tokio::{
    self,
    sync::{Mutex, MutexGuard, OnceCell},
};

use crate::{fs::touch, migrator::Migrator, path::app_data_dir};

/// The static database connection (pool), implemented as an [`OnceCell`]
static DATABASE_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::const_new();

static WRITING_DATABASE_CONNECTION: OnceCell<Mutex<&'static DatabaseConnection>> =
    OnceCell::const_new();

type ReadingDatabaseConnection = &'static DatabaseConnection;

type WritingDatabaseConnection = MutexGuard<'static, ReadingDatabaseConnection>;

/// Get the static database connection (pool).
///
/// Use [`connect_writing`] if you are doing write operations on the connection!
///
/// Migrations are run once on initialization, see [`get_connection_and_init`].
pub async fn connect() -> ReadingDatabaseConnection {
    DATABASE_CONNECTION
        .get_or_init(get_connection_and_init)
        .await
}

/// Get the static database connection (pool) inside a [`MutexGuard`].
///
/// Use [`connect`] if you don't do write operations on the connection!
///
/// Only [one writer can exist at a time](https://www.sqlite.org/wal.html#concurrency).
pub async fn connect_writing() -> WritingDatabaseConnection {
    async fn init() -> Mutex<ReadingDatabaseConnection> {
        let db = connect().await;
        Mutex::new(db)
    }

    let mutex = WRITING_DATABASE_CONNECTION.get_or_init(init).await;
    mutex.lock().await
}

/// Get the path to the SQLite database file.
///
/// # Panics
///
/// This function panics when the [`std::path::PathBuf`] cannot be converted to a [`String`].
async fn get_path() -> String {
    let database_path_buf = app_data_dir().join("database.db");
    if let Err(err) = touch(&database_path_buf).await {
        log::error!("Could not touch {database_path_buf:?}: {err}");
    }
    match database_path_buf.into_os_string().into_string() {
        Ok(ok) => ok,
        Err(err) => {
            let msg = format!("Could not get path to database from os string: \"{err:?}\"",);
            log::error!("{msg}");
            panic!("{msg}");
        }
    }
}

/// Connect to the SQLite database file and return the connection.
///
/// # Panics
///
/// This function panics...
/// - ...when [`get_path`] panics.
/// - ...when no connection to the SQLite database cannot be established.
async fn get_connection() -> DatabaseConnection {
    let path = get_path().await;
    let database_url = String::from("sqlite://") + path.as_str();
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(256)
        .acquire_timeout(Duration::from_secs(69420))
        .sqlx_logging(true)
        .sqlx_logging_level(LevelFilter::Trace);
    match Database::connect(opt).await {
        Ok(ok) => ok,
        Err(err) => {
            let msg = format!("Could not connect to database: {err}",);
            log::error!("{msg}");
            panic!("{msg}");
        }
    }
}

/// Set [WAL mode](https://www.sqlite.org/wal.html) for the database.
async fn set_wal_mode(connection: &DatabaseConnection) {
    if let Err(err) = connection
        .execute_unprepared("PRAGMA journal_mode=WAL;")
        .await
    {
        let msg = format!("Could not set WAL mode: {err}");
        log::error!("{msg}");
        panic!("{msg}");
    }
}

/// Run database migrations in [`crate::migrator`].
async fn migrate(connection: &DatabaseConnection) {
    if let Err(err) = Migrator::up(connection, None).await {
        let msg = format!("Could not migrate database: {err}");
        log::error!("{msg}");
        panic!("{msg}");
    }
}

/// Get the database connection and initialize.
///
/// Returns the database connection.
async fn get_connection_and_init() -> DatabaseConnection {
    let connection = get_connection().await;
    set_wal_mode(&connection).await;
    migrate(&connection).await;
    connection
}

#[cfg(test)]
pub mod tests {
    use sea_orm::{ConnectionTrait, Database, DatabaseConnection, Statement};
    use sea_query::Value;

    pub async fn get_memory_database() -> DatabaseConnection {
        Database::connect("sqlite::memory:").await.unwrap()
    }

    pub async fn get_table_schema(table: &str, db: &DatabaseConnection) -> String {
        let query_result = db
            .query_one(Statement::from_sql_and_values(
                db.get_database_backend(),
                "SELECT `sql` FROM `sqlite_schema` WHERE `tbl_name` = ?;",
                vec![Value::from(table)],
            ))
            .await
            .unwrap()
            .unwrap();
        query_result.try_get("", "sql").unwrap()
    }

    pub async fn get_table_indices(table: &str, db: &DatabaseConnection) -> Vec<String> {
        db.query_all(Statement::from_sql_and_values(
            db.get_database_backend(),
            "SELECT `sql` FROM `sqlite_master` WHERE `tbl_name` = ? AND `type` = \"index\" AND NOT `sql` IS NULL;",
            vec![Value::from(table)],
        ))
        .await
        .unwrap()
        .iter()
        .map(|query_result| query_result.try_get("", "sql").unwrap())
        .collect()
    }
}
