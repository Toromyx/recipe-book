use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use tokio::{self, sync::OnceCell};

use crate::{fs::touch, migrator::Migrator, path::app_data_dir};

static DATABASE_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn connect() -> &'static DatabaseConnection {
    DATABASE_CONNECTION
        .get_or_init(get_connection_and_migrate)
        .await
}

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

async fn get_connection() -> DatabaseConnection {
    let path = get_path().await;
    let database_url = String::from("sqlite://") + &path;
    match Database::connect(database_url).await {
        Ok(ok) => ok,
        Err(err) => {
            let msg = format!("Could not connect to database: {err}",);
            log::error!("{msg}");
            panic!("{msg}");
        }
    }
}

async fn migrate(connection: &DatabaseConnection) {
    if let Err(err) = Migrator::up(connection, None).await {
        log::error!("Could not migrate database: {err}",);
    }
}

async fn get_connection_and_migrate() -> DatabaseConnection {
    let connection = get_connection().await;
    migrate(&connection).await;
    connection
}
