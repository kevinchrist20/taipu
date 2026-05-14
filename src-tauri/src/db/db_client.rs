use std::fs;
use std::path::Path;

use diesel::{Connection, RunQueryDsl, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

// Init db
pub fn init_db() {
    if !db_exist() {
        create_db_file();
    }

    run_migrations();
}

pub fn db_conn() -> SqliteConnection {
    let db_path = get_db_path();

    SqliteConnection::establish(&db_path)
        .unwrap_or_else(|_| panic!("Error connecting to db at: {}", db_path))
}

fn create_db_file() {
    let db_path = get_db_path();
    let db_parent_dir = Path::new(&db_path).parent().unwrap();

    // Create parent dirs if they don't exist
    if !db_parent_dir.exists() {
        fs::create_dir_all(db_parent_dir).unwrap()
    }

    fs::File::create(db_path).unwrap();
}

fn db_exist() -> bool {
    let db_path = get_db_path();
    Path::new(&db_path).exists()
}

fn get_db_path() -> String {
    let home_dir = dirs::home_dir().unwrap();
    home_dir.to_str().unwrap().to_string() + "/.config/taipu/database.sqlite"
}

fn run_migrations() {
    let mut connection = migration_conn();

    // PRAGMA foreign_keys must be set outside a transaction to take effect.
    // Disable it so the users table-rebuild migration can drop the referenced table.
    diesel::sql_query("PRAGMA foreign_keys = OFF")
        .execute(&mut connection)
        .expect("Failed to disable foreign key checks");

    if let Err(e) = connection.run_pending_migrations(MIGRATIONS) {
        eprintln!("Migration error: {e:?}");
        std::process::exit(1);
    }

    diesel::sql_query("PRAGMA foreign_keys = ON")
        .execute(&mut connection)
        .expect("Failed to re-enable foreign key checks");
}

fn migration_conn() -> SqliteConnection {
    let db_path = "sqlite://".to_string() + get_db_path().as_str();

    SqliteConnection::establish(&db_path)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_path))
}
